extern crate alloc;

use core::cmp::Reverse;

use alloc::{vec::Vec, collections::BinaryHeap};

use binmarshal::{BinMarshal, LengthTaggedVec, rw::{BitView, BufferBitWriter, BitWriter}};

use crate::{Message, ManufacturerMessage};

const GRAPPLE_API_CLASS_FRAGMENT: u8 = 0b100000;
const GRAPPLE_API_INDEX_FRAGMENT_START: u8 = 0b0;

#[derive(Debug, Clone, BinMarshal)]
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

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = u8)]
pub enum CANMessage {
  #[marshal(tag = "0")]
  Message(Message),
  #[marshal(tag = "1")]
  Unknown(GenericCANMessage),
  #[marshal(tag = "2")]
  FragmentStart(u8, u8, GenericCANMessage),     // (identifier, fragment_len, message)
  #[marshal(tag = "3")]
  Fragment(u8, u8, GenericCANMessage),          // (identifier, index, message)
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
            payload: GenericCANPayload { payload: LengthTaggedVec::new(buffer[3..].to_vec()) }
          })
        },
        seq => CANMessage::Fragment(x, seq, GenericCANMessage { id: id.clone(), payload: GenericCANPayload { payload: LengthTaggedVec::new(buffer.to_vec()) } })
      },
      _ => {
        // It's part of a normal message
        let manufacturer_msg = ManufacturerMessage::read(&mut BitView::new(buffer), crate::MessageContext { device_type: id.device_type, manufacturer: id.manufacturer, api_class: id.api_class, api_index: id.api_index, device_id: id.device_id });
        match manufacturer_msg {
          Some(manufacturer_msg) => CANMessage::Message(Message::new(id.device_id, manufacturer_msg)),
          None => CANMessage::Unknown(GenericCANMessage { 
            id,
            payload: GenericCANPayload {
              payload: LengthTaggedVec::new(buffer.to_vec())
            }
          })
        }
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CANId {
  pub device_type: u8,
  pub manufacturer: u8,
  pub api_class: u8,
  pub api_index: u8,
  pub device_id: u8,
}

impl BinMarshal<()> for CANId {
  type Context = ();

  fn write<W: binmarshal::rw::BitWriter>(self, writer: &mut W, ctx: ()) -> bool {
    u32::write(self.into(), writer, ctx)
  }

  fn read(view: &mut binmarshal::rw::BitView<'_>, ctx: ()) -> Option<Self> {
    u32::read(view, ctx).map(Into::into)
  }

  fn update<'a>(&'a mut self, _ctx: <() as binmarshal::BinmarshalContext>::MutableComplement<'a>) { }
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

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenericCANMessage {
  pub id: CANId,
  pub payload: GenericCANPayload
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GenericCANPayload {
  pub payload: LengthTaggedVec<u8, u8>
}

pub struct FragmentData {
  seq: u8,
  payload: Vec<u8>,
  target_frame_id: Option<CANId>,
  total_len: Option<u8>
}

impl PartialEq for FragmentData {
  fn eq(&self, other: &Self) -> bool {
    self.seq == other.seq
  }
}

impl Eq for FragmentData {}

impl PartialOrd for FragmentData {
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    self.seq.partial_cmp(&other.seq)
  }
}

impl Ord for FragmentData {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.seq.cmp(&other.seq)
  }
}

#[derive(PartialEq, Eq)]
pub struct FragmentSetKey {
  device_id: u8,
  device_type: u8,
  manufacturer: u8,
  fragment_idx: u8,
}

pub struct Fragments {
  key: FragmentSetKey,
  data: BinaryHeap<Reverse<FragmentData>>,
  last_seen: i64,
}

pub struct FragmentReassembler {
  messages: Vec<Fragments>,
  age_off: i64
}

impl FragmentReassembler {
  pub fn new(age_off: i64) -> Self {
    Self { age_off, messages: Vec::new() }
  }

  fn add_data(&mut self, now: i64, key: FragmentSetKey, fd: FragmentData) -> Option<(u8, CANMessage)> {
    self.messages.retain(|frags| (now - frags.last_seen) <= self.age_off);

    let fragments = match self.messages.iter_mut().find(|x| x.key == key) {
      Some(frags) => frags,
      None => {
        self.messages.push(Fragments { key, data: BinaryHeap::with_capacity(4), last_seen: now });
        let n = self.messages.len();
        self.messages.get_mut(n - 1).unwrap()
      },
    };

    fragments.data.push(Reverse(fd));

    if fragments.data.peek().unwrap().0.seq == 0 {
      let total_count = fragments.data.peek().unwrap().0.total_len.unwrap() as usize;
      let n_bytes_seen = fragments.data.iter().map(|x| x.0.payload.len()).reduce(|a, b| a + b).unwrap_or(0);

      if n_bytes_seen >= total_count {
        // This fragment is complete
        let first = fragments.data.pop().unwrap().0;
        let mut data = Vec::with_capacity(n_bytes_seen);
        data.extend(first.payload);

        while !fragments.data.is_empty() {
          data.extend(fragments.data.pop().unwrap().0.payload);
        }

        Some((n_bytes_seen as u8, CANMessage::decode(first.target_frame_id.unwrap(), &data[..])))
      } else {
        None
      }
    } else {
      None
    }
  }

  pub fn process(&mut self, now: i64, raw_len: u8, message: CANMessage) -> Option<(u8, CANMessage)> {
    let ret = match message {
      CANMessage::Message(_) => Some((raw_len, message)),
      CANMessage::Unknown(_) => Some((raw_len, message)),
      CANMessage::FragmentStart(id, len, frag) => {
        self.add_data(now, FragmentSetKey {
          device_id: frag.id.device_id,
          device_type: frag.id.device_type,
          manufacturer: frag.id.manufacturer,
          fragment_idx: id
        }, FragmentData {
          seq: 0,
          payload: frag.payload.payload.0,
          target_frame_id: Some(frag.id),
          total_len: Some(len)
        })
      },
      CANMessage::Fragment(id, seq, payload) => {
        self.add_data(now, FragmentSetKey {
          device_id: payload.id.device_id,
          device_type: payload.id.device_type,
          manufacturer: payload.id.manufacturer,
          fragment_idx: id
        }, FragmentData {
          seq,
          payload: payload.payload.payload.0,
          target_frame_id: None,
          total_len: None
        })
      },
    };

    return ret;
  }

  pub fn maybe_split(message: Message, fragment_id: u8) -> Option<smallvec::SmallVec<[UnparsedCANMessage; 4]>> {
    let mut payload = [0u8; 256];
    let mut writer = BufferBitWriter::new(&mut payload);
    let success = message.msg.write(&mut writer, crate::MessageContext { device_type: message.device_type, manufacturer: message.manufacturer, api_class: message.api_class, api_index: message.api_index, device_id: message.device_id });
    if !success {
      return None;
    }

    let payload_slice = writer.slice();
    // let mut payload = bitvec![u8, Msb0;];
    // message.msg.write(&mut payload, (message.device_type, message.manufacturer, message.api_class, message.api_index))?;
    // let payload_slice = payload.as_raw_slice();

    if payload_slice.len() > 8 as usize {
      // Requires split
      let mut msgs: smallvec::SmallVec<[UnparsedCANMessage; 4]> = smallvec::smallvec![];

      // First message - first three bytes are frag api class, frag api idx, and total size followed by the first 5 bytes of the fragment
      msgs.push(UnparsedCANMessage {
        id: CANId {
          device_type: message.device_type,
          manufacturer: message.manufacturer,
          api_class: (fragment_id & 0b11111) | GRAPPLE_API_CLASS_FRAGMENT,
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
            api_class: (fragment_id & 0b11111) | GRAPPLE_API_CLASS_FRAGMENT,
            api_index: i,
            device_id: message.device_id
          },
          len: n as u8,
          payload: buf
        });

        offset += 8;
        i += 1;
      }

      Some(msgs)
    } else {
      // Can send straight up
      let len = payload_slice.len();
      let mut buf = [0u8; 8];
      
      for i in 0..len {
        buf[i] = payload_slice[i];
      }

      Some(smallvec::smallvec![UnparsedCANMessage {
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
mod test {
    use crate::can::CANMessage;
    use crate::{Message, DEVICE_ID_BROADCAST};

    use super::FragmentReassembler;

    use rand::thread_rng;
    use rand::seq::SliceRandom;

  #[test]
  fn test_reassemble() {
    let msg = Message::new(DEVICE_ID_BROADCAST, crate::ManufacturerMessage::Grapple(crate::grapple::GrappleDeviceMessage::Broadcast(
      crate::grapple::GrappleBroadcastMessage::DeviceInfo(crate::grapple::device_info::GrappleDeviceInfo::SetName {
        serial: 0xDEADBEEF,
        name: "Some Really Really Long Name".to_owned() })
    )));

    let msgs = FragmentReassembler::maybe_split(msg.clone(), 0x12);
    let mut msgs = msgs.unwrap().to_vec();
    msgs.shuffle(&mut thread_rng());

    let mut reassembler = FragmentReassembler::new(200);

    let mut out = None;
    for msg in msgs {
      out = reassembler.process(0, msg.len, CANMessage::decode(msg.id, &msg.payload[0..msg.len as usize]));
    }

    assert_eq!(out.map(|(_, msg)| msg), Some(CANMessage::Message(msg.clone())));

    let msg = Message::new(DEVICE_ID_BROADCAST, crate::ManufacturerMessage::Grapple(crate::grapple::GrappleDeviceMessage::Broadcast(
      crate::grapple::GrappleBroadcastMessage::DeviceInfo(crate::grapple::device_info::GrappleDeviceInfo::SetName {
        serial: 0xDEADBEEF,
        name: "Something Else".to_owned() })
    )));

    let msgs = FragmentReassembler::maybe_split(msg.clone(), 0x12);
    let mut msgs = msgs.unwrap().to_vec();
    msgs.shuffle(&mut thread_rng());

    let mut out = None;
    for msg in msgs {
      out = reassembler.process(0, msg.len, CANMessage::decode(msg.id, &msg.payload[0..msg.len as usize]));
    }

    assert_eq!(out.map(|(_, msg)| msg), Some(CANMessage::Message(msg)));
  }
}