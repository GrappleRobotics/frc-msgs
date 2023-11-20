extern crate alloc;

use deku::{prelude::*, bitvec::{BitSlice, bitvec, Msb0}};
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use alloc::collections::BTreeMap;

use crate::{Message, ManufacturerMessage};

const GRAPPLE_API_CLASS_FRAGMENT: u8 = 0b100000;
const GRAPPLE_API_INDEX_FRAGMENT_START: u8 = 0b0;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct UnparsedCANMessage {
  pub id: CANId,
  pub payload: [u8; 8],
  pub len: u8
}

impl UnparsedCANMessage {
  pub fn new(id: u32, buffer: &[u8]) -> Self {
    let mut s = Self {
      id: CANId::from(id),
      payload: [0u8; 8],
      len: buffer.len() as u8
    };
    s.payload[0..buffer.len()].copy_from_slice(buffer);
    s
  }
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(type = "u8")]
pub enum CANMessage {
  #[deku(id = "0")]
  Message(Message),
  #[deku(id = "1")]
  Unknown(GenericCANMessage),
  #[deku(id = "2")]
  FragmentStart(u8, u8, GenericCANMessage),     // (identifier, fragment_len, message)
  #[deku(id = "3")]
  Fragment(u8, GenericCANPayload),          // (identifier, message)
}

impl From<UnparsedCANMessage> for CANMessage {
  fn from(value: UnparsedCANMessage) -> Self {
    Self::decode(value.id, &value.payload[0..value.len as usize])
  }
}

impl CANMessage {
  pub fn decode(id: CANId, buffer: &[u8]) -> CANMessage {
    match (id.manufacturer, id.api_class) {
      (6, x) if x & GRAPPLE_API_CLASS_FRAGMENT != 0 => match id.api_index {
        // It's part of a fragmented message
        GRAPPLE_API_INDEX_FRAGMENT_START => {
          let fragment_api_class = buffer[0];
          let fragment_api_index = buffer[1];
          let message_len = buffer[2];
          CANMessage::FragmentStart(x, message_len, GenericCANMessage {
            id: CANId { device_type: id.device_type, manufacturer: id.manufacturer, api_class: fragment_api_class, api_index: fragment_api_index, device_id: id.device_id },
            payload: GenericCANPayload { payload_len: (buffer.len() - 3) as u8, payload: buffer[3..].to_vec() }
          })
        },
        seq => CANMessage::Fragment(x, GenericCANPayload { payload_len: buffer.len() as u8, payload: buffer.to_vec() })
      },
      _ => {
        // It's part of a normal message
        let manufacturer_msg = ManufacturerMessage::read(BitSlice::from_slice(buffer), (id.device_type, id.manufacturer, id.api_class, id.api_index));
        match manufacturer_msg {
          Ok(manufacturer_msg) => CANMessage::Message(Message::new(id.device_id, manufacturer_msg.1)),
          Err(_) => CANMessage::Unknown(GenericCANMessage { 
            id,
            payload: GenericCANPayload {
              payload_len: buffer.len() as u8,
              payload: buffer.to_vec() 
            }
          })
        }
      }
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CANId {
  pub device_type: u8,
  pub manufacturer: u8,
  pub api_class: u8,
  pub api_index: u8,
  pub device_id: u8,
}

impl DekuWrite for CANId {
    fn write(&self, output: &mut deku::bitvec::BitVec<u8, deku::bitvec::Msb0>, ctx: ()) -> Result<(), DekuError> {
      Into::<u32>::into(self.clone()).write(output, ctx)
    }
}

impl<'a> DekuRead<'a> for CANId {
    fn read(input: &'a deku::bitvec::BitSlice<u8, deku::bitvec::Msb0>, ctx: ()) -> Result<(&'a deku::bitvec::BitSlice<u8, deku::bitvec::Msb0>, Self), DekuError>
    where
      Self: Sized
    {
      let num = u32::read(input, ctx)?;
      Ok((num.0, CANId::from(num.1)))
    }
}

impl From<u32> for CANId {
  fn from(value: u32) -> Self {
    Self {
      device_type: ((value >> 6+4+6+8) & 0b11111) as u8,
      manufacturer: ((value >> 6+4+6) & 0b11111111) as u8,
      api_class: ((value >> 6+4) & 0b111111) as u8,
      api_index: ((value >> 6) & 0b1111) as u8,
      device_id: (value & 0b111111) as u8
    }
  }
}

impl Into<u32> for CANId {
  fn into(self) -> u32 {
    (self.device_id as u32 & 0b1111111)
    | ((self.api_index as u32 & 0b1111) << 6)
    | ((self.api_class as u32 & 0b111111) << (6+4))
    | ((self.manufacturer as u32 & 0b11111111) << (6+4+6))
    | ((self.device_type as u32 & 0b11111) << (6+4+6+8))
  }
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenericCANMessage {
  pub id: CANId,
  pub payload: GenericCANPayload
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenericCANPayload {
  #[deku(update = "self.payload.len()")]
  pub payload_len: u8,
  #[deku(count = "payload_len")]
  pub payload: Vec<u8>
}

pub struct FragmentMetadata {
  last_update: i64,
  id: CANId,
  len: u8,
  payload: Vec<u8>
}

pub struct FragmentReassembler {
  messages: BTreeMap<u8, FragmentMetadata>,
  age_off: i64
}

impl FragmentReassembler {
  pub fn new(age_off: i64) -> Self {
    Self { age_off, messages: BTreeMap::new() }
  }

  pub fn process(&mut self, now: i64, raw_len: u8, message: CANMessage) -> Option<(u8, CANMessage)> {
    let ret = match message {
      CANMessage::Message(_) => Some((raw_len, message)),
      CANMessage::Unknown(_) => Some((raw_len, message)),
      CANMessage::FragmentStart(id, len, frag) => {
        let mut meta = FragmentMetadata {
          last_update: now,
          id: frag.id,
          len,
          payload: Vec::with_capacity(len as usize)
        };
        meta.payload.extend(frag.payload.payload);
        self.messages.insert(id, meta);
        None
      },
      CANMessage::Fragment(id, payload) => {
        let is_done = {
          let meta = self.messages.get_mut(&id);
          if let Some(meta) = meta {
            meta.last_update = now;
            meta.payload.extend(payload.payload);
            meta.payload.len() >= meta.len as usize
          } else { false }
        };

        if is_done {
          // Reassemble
          let meta = self.messages.remove(&id).unwrap();
          let decoded = CANMessage::decode(meta.id.clone(), &meta.payload[0..meta.len as usize]);
          Some((meta.len + 3, decoded))
        } else {
          None
        }
      },
    };

    // Get rid of anything that's aged off
    self.messages.retain(|_, v| (now - v.last_update) <= self.age_off);

    return ret;
  }

  pub fn maybe_split(message: Message, fragment_id: u8) -> Result<smallvec::SmallVec<[UnparsedCANMessage; 4]>, DekuError> {
    let mut payload = bitvec![u8, Msb0;];
    message.msg.write(&mut payload, (message.device_type, message.manufacturer, message.api_class, message.api_index))?;
    let payload_slice = payload.as_raw_slice();

    if payload_slice.len() > 8 as usize {
      // Requires split
      let mut msgs: smallvec::SmallVec<[UnparsedCANMessage; 4]> = smallvec::smallvec![];

      // First message - first three bytes are frag api class, frag api idx, and total size followed by the first 5 bytes of the fragment
      msgs.push(UnparsedCANMessage {
        id: CANId {
          device_type: message.device_type,
          manufacturer: message.manufacturer,
          api_class: fragment_id | GRAPPLE_API_CLASS_FRAGMENT,
          api_index: GRAPPLE_API_INDEX_FRAGMENT_START,
          device_id: message.device_id
        },
        len: 8,
        payload: [
          message.api_class,
          message.api_index,
          payload_slice.len() as u8,
          payload_slice[0], payload_slice[1],
          payload_slice[2], payload_slice[3],
          payload_slice[4]
        ]
      });

      // Subsequent messages are made entirely of the payload
      let mut i = 1;
      let mut offset = 5;
      while offset < payload_slice.len() {
        let mut buf = [0u8; 8];
        let n = 8.min(payload_slice.len() - offset);
        buf[0..n].copy_from_slice(&payload_slice[offset..offset+n]);

        msgs.push(UnparsedCANMessage {
          id: CANId {
            device_type: message.device_type,
            manufacturer: message.manufacturer,
            api_class: fragment_id | GRAPPLE_API_CLASS_FRAGMENT,
            api_index: i,
            device_id: message.device_id
          },
          len: n as u8,
          payload: buf
        });

        offset += 8;
        i += 1;
      }

      Ok(msgs)
    } else {
      // Can send straight up
      let len = payload_slice.len();
      let mut buf = [0u8; 8];
      
      for i in 0..len {
        buf[i] = payload_slice[i];
      }

      Ok(smallvec::smallvec![UnparsedCANMessage {
        id: CANId {
          device_type: message.device_type,
          manufacturer: message.manufacturer,
          api_class: message.api_class,
          api_index: message.api_index,
          device_id: message.device_id
        },
        len: len as u8,
        payload: buf
      }])
    }

  }
}

#[cfg(test)]
mod tests {
  use crate::{Message, can::CANMessage};

  use super::FragmentReassembler;

  #[test]
  fn test_reassemble() {
    let name = "Some long name".to_owned();
    let msg = Message::new(0xAB, crate::ManufacturerMessage::Grapple(crate::grapple::GrappleDeviceMessage::Broadcast(
      crate::grapple::GrappleBroadcastMessage::DeviceInfo(
        crate::grapple::device_info::GrappleDeviceInfo::EnumerateResponse {
          model_id: crate::grapple::device_info::GrappleModelId::SpiderLan,
          firmware_version: [1, 2, 3],
          serial: 0xdeadbeef,
          is_dfu: false,
          is_dfu_in_progress: false,
          reserved: 0,
          name_len: name.len() as u8,
          name: name.as_bytes().to_vec()
        }
      )
    )));

    println!("{:?}", msg);

    let mut reassembler = FragmentReassembler::new(1000);
    let fragments = FragmentReassembler::maybe_split(msg.clone(), 12).unwrap();

    let mut result = None;
    for frag in fragments {
      println!("{:?}", frag);
      result = reassembler.process(0, frag.len, frag.into());
    }
    match result.unwrap().1 {
      CANMessage::Message(m) if m == msg => (),
      _ => panic!("Defragmented Message does not match!")
    }
  }
}