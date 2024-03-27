use crate::Validate;
use binmarshal::{Marshal, Demarshal, MarshalUpdate, AsymmetricCow};
use bounded_static::ToStatic;

use super::{GrappleMessageId, errors::GrappleResult};

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = u8)]
#[repr(u8)]
pub enum GrappleModelId {
  #[marshal(tag = "0x10")]
  LaserCan = 0x10,
  #[marshal(tag = "0x20")]
  SpiderLan = 0x20,
  #[marshal(tag = "0x30")]
  #[allow(non_camel_case_types)]
  // Codeword for a future product :)
  ACROBATIC_LEMUR = 0x30,
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_index")]
pub enum GrappleDeviceInfo<'a> {
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
    version: AsymmetricCow<'a, str>,

    name: AsymmetricCow<'a, str>
  },

  #[marshal(tag = "2")]
  Blink {
    serial: u32
  },

  #[marshal(tag = "3")]
  SetName {
    serial: u32,
    name: AsymmetricCow<'a, str>
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

impl<'a> Validate for GrappleDeviceInfo<'a> {
  fn validate(&self) -> GrappleResult<()> {
    Ok(())
  }
}
