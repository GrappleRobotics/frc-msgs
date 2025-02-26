use core::ops::{Index, RangeFull};

use alloc::borrow::Cow;
use binmarshal::{BitView, BufferBitWriter, BitWriter, Payload, Marshal, Demarshal, MarshalUpdate, AsymmetricCow};
use bounded_static::ToStatic;
use smallvec::SmallVec;

use crate::MessageId;

use super::{GrappleMessageId, MaybeFragment, GrappleDeviceMessage, MANUFACTURER_GRAPPLE};

#[derive(Debug, Clone, PartialEq, Eq, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum FragmentBody {
  Start {
    api_class: u8,
    api_index: u8,
    total_len: u8,
  },
  Fragment
}

#[derive(Debug, Clone, PartialEq, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Fragment<'a> {
  fragment_id: u8,
  index: u8,
  body: FragmentBody,
  payload: AsymmetricCow<'a, Payload>,
}

impl<'a> Marshal<GrappleMessageId> for Fragment<'a> {
  fn write<W: BitWriter>(&self, writer: &mut W, _ctx: GrappleMessageId) -> Result<(), binmarshal::MarshalError> {
    match self.body {
      FragmentBody::Start { api_class, api_index, total_len } => {
        let buf = writer.reserve_and_advance_aligned_slice(3)?;
        buf[0] = api_class;
        buf[1] = api_index;
        buf[2] = total_len;
      },
      FragmentBody::Fragment => (),
    };

    self.payload.write(writer, ())?;

    Ok(())
  }
}

impl<'dm> Demarshal<'dm, GrappleMessageId> for Fragment<'dm> {
  fn read(view: &mut BitView<'dm>, ctx: GrappleMessageId) -> Result<Self, binmarshal::MarshalError> {
    match ctx.api_index {
      0b0 => {
        let header = view.take_aligned_slice(3)?;
        let api_class = header[0];
        let api_index = header[1];
        let total_len = header[2];

        Ok(Fragment {
          fragment_id: ctx.api_class,
          index: 0,
          body: FragmentBody::Start { api_class, api_index, total_len },
          payload: Demarshal::read(view, ())?
        })
      },
      index => {
        Ok(Fragment {
          fragment_id: ctx.api_class,
          index,
          body: FragmentBody::Fragment,
          payload: Demarshal::read(view, ())?
        })
      }
    }
  }
}

impl<'a> MarshalUpdate<GrappleMessageId> for Fragment<'a> {
  fn update(&mut self, ctx: &mut GrappleMessageId) {
    ctx.fragment_flag = true;
    ctx.ack_flag = ctx.ack_flag;
    ctx.api_class = self.fragment_id;
    ctx.api_index = self.index;
  }
}

#[derive(PartialEq, Eq)]
pub struct FragmentSetKey {
  device_id: u8,
  device_type: u8,
  fragment_idx: u8,
}

pub struct Fragments {
  key: FragmentSetKey,
  header: Option<(/* api class */ u8, /* api index */ u8, /* total len */ u8)>,
  data: smallvec::SmallVec<[Option<smallvec::SmallVec<[u8; 8]>>; 8]>,
  last_seen: i64,
}

pub struct FragmentReassembler {
  rx: FragmentReassemblerRx,
  tx: FragmentReassemblerTx
}

impl FragmentReassembler {
  pub fn new(age_off: i64, max_fragment_size: usize) -> Self {
    Self { rx: FragmentReassemblerRx { messages: alloc::vec![], age_off }, tx: FragmentReassemblerTx { max_fragment_size, fragment_id: 0 } }
  }

  pub fn split(self) -> (FragmentReassemblerRx, FragmentReassemblerTx) {
    (self.rx, self.tx)
  }
}

pub struct FragmentReassemblerRx {
  messages: alloc::vec::Vec<Fragments>,
  age_off: i64,
}

pub struct FragmentReassemblerTx {
  max_fragment_size: usize,
  fragment_id: u8,
}

impl FragmentReassemblerRx {
  pub fn defragment<'a, E: Extend<u8> + Index<RangeFull, Output = [u8]>>(&mut self, now: i64, id: &MessageId, message: MaybeFragment<'a>, storage: &'a mut E) -> Result<Option<GrappleDeviceMessage<'a>>, binmarshal::MarshalError> {
    self.messages.retain(|frags| (now - frags.last_seen) <= self.age_off);

    match message {
      MaybeFragment::Fragment(frag) => {
        let key = FragmentSetKey {
          device_id: id.device_id,
          device_type: id.device_type,
          fragment_idx: frag.fragment_id,
        };

        // Find or insert
        let idx = match self.messages.iter().position(|x| x.key == key) {
          Some(idx) => idx,
          None => {
            self.messages.push(Fragments { key, header: None, data: smallvec::SmallVec::with_capacity(2), last_seen: now });
            let n = self.messages.len();
            n - 1
          }
        };

        let fragments = self.messages.get_mut(idx).unwrap();
        fragments.last_seen = now;

        let seq = frag.index as usize;
        if seq >= fragments.data.len() {
          fragments.data.resize(seq as usize + 1, None);
        }

        match frag.body {
          FragmentBody::Start { api_class, api_index, total_len } => {
            fragments.header = Some((api_class, api_index, total_len));
          },
          FragmentBody::Fragment => {},
        }

        fragments.data[seq] = {
          let mut data = smallvec::smallvec![0u8; frag.payload.len()];
          data.copy_from_slice(&frag.payload[..]);
          Some(data)
        };

        let ret = match fragments.header {
          Some((api_class, api_index, total_len)) => {
            let n_bytes_seen = fragments.data.iter().fold(0usize, |a, b| a + b.as_ref().map(|x| x.len()).unwrap_or(0usize));

            if n_bytes_seen >= total_len as usize {
              // Fragment is complete - reassemble it
              for el in &fragments.data {
                if let Some(el) = el.as_ref() {
                  storage.extend(el.iter().cloned());
                }
              }

              let mut view = BitView::new(&storage[..]);
              let msg = GrappleDeviceMessage::read(&mut view, MessageId {
                device_type: id.device_type,
                manufacturer: MANUFACTURER_GRAPPLE,
                api_class,
                api_index,
                device_id: id.device_id,
              }.into());

              match msg {
                Ok(msg) => Ok(Some(msg)),
                Err(e) => Err(e),
              }
            } else {
              Ok(None)
            }
          },
          None => Ok(None)
        };

        match ret {
          Ok(Some(v)) => {
            self.messages.remove(idx);
            Ok(Some(v))
          },
          Ok(None) => Ok(None),
          Err(e) => {
            self.messages.remove(idx);
            Err(e)
          },
        }
      },
      MaybeFragment::Message(msg) => {
        Ok(Some(msg))
      },
    }
  }
}

impl FragmentReassemblerTx {
  pub fn set_fragment_size(&mut self, size: usize) {
    self.max_fragment_size = size;
  }

  pub fn maybe_fragment<Consumer: FnMut(MessageId, &[u8])>(&mut self, device_id: u8, mut message: GrappleDeviceMessage, consumer: &mut Consumer) -> Result<(), binmarshal::MarshalError> {
    let mut payload = [0u8; 253];
    let mut writer = BufferBitWriter::new(&mut payload);
    
    let mut id = GrappleMessageId::new(device_id);
    message.update(&mut id);

    message.write(&mut writer, id.clone())?;

    let payload_slice = writer.slice();
    
    if payload_slice.len() <= self.max_fragment_size {
      consumer(id.into(), payload_slice)
    } else {
      self.fragment_id = self.fragment_id.wrapping_add(1);

      let first_size = self.max_fragment_size - 3;

      let mut first = Fragment {
        fragment_id: self.fragment_id,
        index: 0,
        body: FragmentBody::Start { api_class: id.api_class, api_index: id.api_index, total_len: payload_slice.len() as u8 },
        payload: Cow::Borrowed(Into::<&Payload>::into(&payload_slice[0..first_size])).into()
      };

      // Serialise the first fragment, including an ID update
      let mut id2 = GrappleMessageId::new(device_id);
      id2.device_type = id.device_type;
      first.update(&mut id2);
      let mut buf: SmallVec<[u8; 8]> = smallvec::smallvec![0u8; self.max_fragment_size];
      let mut buf_writer = BufferBitWriter::new(&mut buf);
      first.write(&mut buf_writer, id2.clone())?;

      consumer(id2.into(), buf_writer.slice());

      let mut i = 1;
      let mut offset = first_size;
      while offset < payload_slice.len() {
        let n = self.max_fragment_size.min(payload_slice.len() - offset);

        let mut frag = Fragment {
          fragment_id: self.fragment_id,
          index: i,
          body: FragmentBody::Fragment,
          payload: Cow::Borrowed(Into::<&Payload>::into(&payload_slice[offset..offset + n])).into()
        };

        // Serialise remaining fragments, including an ID update
        let mut id3 = GrappleMessageId::new(device_id);
        id3.device_type = id.device_type;
        frag.update(&mut id3);
        let mut buf = [0u8; 8];
        let mut buf_writer = BufferBitWriter::new(&mut buf);
        frag.write(&mut buf_writer, id3.clone())?;

        consumer(id3.into(), buf_writer.slice());

        offset += 8;
        i += 1;
      }
    }

    Ok(())
  }
}