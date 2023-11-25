use crate::MessageContext;
use binmarshal::{BinMarshal, LengthTaggedVec};

#[derive(Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
pub enum GrappleModelId {
  #[marshal(tag = "0x10")]
  LaserCan,
  #[marshal(tag = "0x20")]
  SpiderLan,
}

#[derive(Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageContext, tag = "ctx.api_index")]
pub enum GrappleDeviceInfo {
  #[marshal(tag = "0")]
  EnumerateRequest,

  #[marshal(tag = "1")]
  EnumerateResponse {
    model_id: GrappleModelId,
    firmware_version: [u8; 3],
    serial: u32,

    #[marshal(bits = 1)]
    is_dfu: bool,
    #[marshal(bits = 1)]
    is_dfu_in_progress: bool,

    #[marshal(align = 1)]
    name: LengthTaggedVec<u8, u8>
  },

  #[marshal(tag = "2")]
  Blink {
    serial: u32
  },

  #[marshal(tag = "3")]
  SetName {
    serial: u32,
    name: LengthTaggedVec<u8, u8>
  },

  #[marshal(tag = "4")]
  CommitConfig {
    serial: u32
  },

  #[marshal(tag = "5")]
  SetId {
    serial: u32,
    new_id: u8
  }
}
