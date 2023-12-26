use crate::Validate;
use alloc::string::String;
use binmarshal::BinMarshal;

use super::{GrappleMessageId, errors::GrappleResult};

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = u8)]
#[repr(u8)]
pub enum GrappleModelId {
  #[marshal(tag = "0x10")]
  LaserCan = 0x10,
  #[marshal(tag = "0x20")]
  SpiderLan = 0x20,
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_index")]
pub enum GrappleDeviceInfo {
  #[marshal(tag = "0")]
  EnumerateRequest,

  #[marshal(tag = "1")]
  EnumerateResponse {
    model_id: GrappleModelId,
    serial: u32,

    #[marshal(bits = 1)]
    is_dfu: bool,
    #[marshal(bits = 1)]
    is_dfu_in_progress: bool,

    #[marshal(align = 1)]
    version: String,

    name: String
  },

  #[marshal(tag = "2")]
  Blink {
    serial: u32
  },

  #[marshal(tag = "3")]
  SetName {
    serial: u32,
    name: String
  },

  #[marshal(tag = "4")]
  CommitConfig {
    serial: u32
  },

  #[marshal(tag = "5")]
  SetId {
    serial: u32,
    new_id: u8
  },

  #[marshal(tag = "6")]
  ArbitrationRequest,

  #[marshal(tag = "7")]
  ArbitrationReject,
}

impl Validate for GrappleDeviceInfo {
  fn validate(&self) -> GrappleResult<()> {
    Ok(())
  }
}
