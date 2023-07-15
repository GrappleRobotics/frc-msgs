extern crate alloc;
use deku::prelude::*;
use alloc::format;
use alloc::vec::Vec;

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_class: u8, api_index: u8", id = "api_class")]
pub enum GrappleFirmwareMessage {
  #[deku(id = "0")]
  StartFieldUpgrade { serial: u32 },
  #[deku(id = "1")]
  UpdatePart {
    serial: u32,
    #[deku(update = "data.len()")]
    len: u8,
    #[deku(count = "len")]
    data: Vec<u8>
  },
  #[deku(id = "2")]
  UpdatePartAck { serial: u32 },
  #[deku(id = "3")]
  UpdateDone { serial: u32 }
}

impl GrappleFirmwareMessage {
  pub fn api(&self) -> (u8, u8) {
    (
      self.deku_id().unwrap(),
      match self {
        GrappleFirmwareMessage::StartFieldUpgrade { .. } => 0,
        GrappleFirmwareMessage::UpdatePart { .. } => 0,
        GrappleFirmwareMessage::UpdatePartAck { .. } => 0,
        GrappleFirmwareMessage::UpdateDone { .. } => 0,
      }
    )
  }
}