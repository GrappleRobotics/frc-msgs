extern crate alloc;
use deku::prelude::*;
use alloc::vec::Vec;
use alloc::format;

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(type = "u8")]
pub enum GrappleModelId {
  LaserCan = 0x00,
  SpiderLan = 0x10,
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_index: u8", id = "api_index")]
pub enum GrappleDeviceInfo {
  #[deku(id = "0")]
  EnumerateRequest,

  #[deku(id = "1")]
  EnumerateResponse {
    model_id: GrappleModelId,
    firmware_version: [u8; 3],
    serial: u32,

    #[deku(bits = "1")]
    is_dfu: bool,
    #[deku(bits = "1")]
    is_dfu_in_progress: bool,
    #[deku(bits = "6")]
    reserved: u8,

    #[deku(assert = "*name_len <= 16", update = "name.len()")]
    name_len: u8,
    #[deku(count = "name_len")]
    name: Vec<u8>
  },

  #[deku(id = "2")]
  Blink {
    serial: u32
  },

  #[deku(id = "3")]
  SetName {
    serial: u32,
    #[deku(assert = "*name_len <= 16")]
    name_len: u8,
    #[deku(count = "name_len")]
    name: Vec<u8>
  },

  #[deku(id = "4")]
  CommitConfig {
    serial: u32
  },

  #[deku(id = "5")]
  SetId {
    serial: u32,
    #[deku(assert = "*new_id <= (0x3F - 1)")]
    new_id: u8
  }
}
