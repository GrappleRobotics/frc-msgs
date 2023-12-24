use binmarshal::{BinMarshal, BitView, BufferBitWriter, BitWriter};

use crate::MessageId;

use super::{GrappleMessageId, MaybeFragment, GrappleDeviceMessage, MANUFACTURER_GRAPPLE};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Fragment {
  fragment_id: u8,
  index: u8,
  body: FragmentBody,
  payload: [u8; 8],
  len: u8
}

impl BinMarshal<GrappleMessageId> for Fragment {
  type Context = GrappleMessageId;

  fn write<W: binmarshal::BitWriter>(self, writer: &mut W, _ctx: GrappleMessageId) -> bool {
    (match self.body {
      FragmentBody::Start { api_class, api_index, total_len } => {
        match writer.reserve_and_advance_aligned_slice(8) {
          Some(buf) => {
            buf[0] = api_class;
            buf[1] = api_index;
            buf[2] = total_len;
            true
          },
          None => false
        }
      },
      FragmentBody::Fragment => true,
    }) && (match writer.reserve_and_advance_aligned_slice(self.len as usize) {
      Some(buf) => {
        buf.copy_from_slice(&self.payload[0..self.len as usize]);
        true
      },
      None => false,
    })
  }

  fn read(view: &mut binmarshal::BitView<'_>, ctx: GrappleMessageId) -> Option<Self> {
    match ctx.api_index {
      0b0 => match view.take::<8>(8, 0) {
        Some((buf, _)) => {
          Some(Fragment {
            fragment_id: ctx.api_class,
            index: 0,
            len: 5,
            payload: [ buf[3], buf[4], buf[5], buf[6], buf[7], 0, 0, 0 ],
            body: FragmentBody::Start {
              api_class: buf[0],
              api_index: buf[1],
              total_len: buf[3],
            }
          })
        },
        None => None,
      },
      index => {
        let mut payload = [0u8; 8];
        let mut i = 0u8;

        while i < 8 {
          match view.take::<1>(1, 0) {
            Some((buf, _)) => {
              payload[i as usize] = buf[0];
              i += 1;
            },
            None => break,
          }
        }

        Some(Fragment {
          fragment_id: ctx.api_class,
          index,
          payload,
          len: i,
          body: FragmentBody::Fragment
        })
      }
    }
  }

  fn update(&mut self, ctx: &mut GrappleMessageId) {
    ctx.fragment_flag = true;
    ctx.ack_flag = false;
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
  data: alloc::vec::Vec<Option<Fragment>>,
  last_seen: i64,
}

pub struct FragmentReassembler {
  messages: alloc::vec::Vec<Fragments>,
  age_off: i64
}

impl FragmentReassembler {
  pub fn new(age_off: i64) -> Self {
    Self { age_off, messages: alloc::vec::Vec::new() }
  }

  pub fn defragment(&mut self, now: i64, id: &MessageId, message: MaybeFragment) -> Option<GrappleDeviceMessage> {
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
            self.messages.push(Fragments { key, data: alloc::vec![None; 4], last_seen: now });
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
        fragments.data[seq] = Some(frag);

        let ret = match fragments.data.get(0) {
          Some(Some(first)) => {
            match first.body {
              FragmentBody::Start { api_class, api_index, total_len } => {
                let n_bytes_seen = fragments.data.iter().fold(0usize, |a, b| a + b.as_ref().map(|x| x.len as usize).unwrap_or(0));

                if n_bytes_seen >= total_len as usize {
                  // Fragment is complete
                  let mut data = alloc::vec::Vec::with_capacity(n_bytes_seen);

                  for el in &fragments.data {
                    if let Some(el) = el.as_ref() {
                      data.extend(&el.payload[0..el.len as usize]);
                    }
                  }

                  let mut view = BitView::new(&data[..]);
                  GrappleDeviceMessage::read(&mut view, MessageId {
                    device_type: id.device_type,
                    manufacturer: MANUFACTURER_GRAPPLE,
                    api_class,
                    api_index,
                    device_id: id.device_id,
                  }.into())
                } else {
                  None
                }
              },
              FragmentBody::Fragment => None,
            }
          },
          _ => None
        };

        if ret.is_some() {
          self.messages.remove(idx);
        }

        ret
      },
      MaybeFragment::Message(msg) => Some(msg),
    }
  }

  pub fn maybe_fragment<Consumer: FnMut(MessageId, &[u8])>(device_id: u8, mut message: GrappleDeviceMessage, fragment_id: u8, consumer: &mut Consumer) {
    let mut payload = [0u8; 253];
    let mut writer = BufferBitWriter::new(&mut payload);
    
    let mut id = GrappleMessageId::new(device_id);
    message.update(&mut id);

    let success = message.write(&mut writer, id.clone());
    if !success {
      return;
    }

    let payload_slice = writer.slice();
    
    if payload_slice.len() <= 8 {
      consumer(id.into(), payload_slice)
    } else {
      let mut first = Fragment {
        fragment_id,
        index: 0,
        body: FragmentBody::Start { api_class: id.api_class, api_index: id.api_index, total_len: payload_slice.len() as u8 },
        payload: [ payload_slice[0], payload_slice[1], payload_slice[2], payload_slice[3], payload_slice[4], 0, 0, 0 ],
        len: 5
      };

      // Serialise the first fragment, including an ID update
      let mut id2 = GrappleMessageId::new(device_id);
      first.update(&mut id2);
      let mut buf = [0u8; 8];
      let mut buf_writer = BufferBitWriter::new(&mut buf);
      first.write(&mut buf_writer, id2.clone());

      consumer(id2.into(), buf_writer.slice());

      let mut i = 1;
      let mut offset = 5;
      while offset < payload_slice.len() {
        let mut buf = [0u8; 8];
        let n = 8.min(payload_slice.len() - offset);
        buf[0..n].copy_from_slice(&payload_slice[offset..offset + n]);

        let mut frag = Fragment {
          fragment_id,
          index: i,
          body: FragmentBody::Fragment,
          payload: buf,
          len: n as u8,
        };

        // Serialise remaining fragments, including an ID update
        let mut id3 = GrappleMessageId::new(device_id);
        frag.update(&mut id3);
        let mut buf = [0u8; 8];
        let mut buf_writer = BufferBitWriter::new(&mut buf);
        frag.write(&mut buf_writer, id3.clone());

        consumer(id3.into(), buf_writer.slice());

        offset += 8;
        i += 1;
      }
    }
  }
}