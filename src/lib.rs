#![cfg_attr(all(not(feature="std"), not(test)), no_std)]

extern crate alloc;

pub mod can;

pub mod grapple;
pub mod ni;

pub use binmarshal;

use binmarshal::{BinMarshal, Context};
use grapple::MANUFACTURER_GRAPPLE;
use grapple::GrappleDeviceMessage;

pub const DEVICE_TYPE_BROADCAST: u8 = 0x00;
pub const DEVICE_TYPE_FIRMWARE_UPGRADE: u8 = 31;
pub const DEVICE_ID_BROADCAST: u8 = 0x3F;

#[derive(Context)]
pub struct MessageContext {
  device_type: u8,
  manufacturer: u8,
  api_class: u8,
  api_index: u8,
  #[allow(dead_code)]
  device_id: u8,
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Message {
  pub device_type: u8,
  pub manufacturer: u8,
  pub api_class: u8,
  pub api_index: u8,

  pub device_id: u8,

  #[marshal(ctx = "{ device_type, manufacturer, api_class, api_index, device_id }")]
  pub msg: ManufacturerMessage
}

impl Message {
  pub fn new(device_id: u8, msg: ManufacturerMessage) -> Self {
    let mut newmsg = Self {
      device_type: 0,
      manufacturer: 0,
      api_class: 0,
      api_index: 0,
      device_id,
      msg,
    };

    newmsg.update(());

    newmsg
  }
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageContext, tag = "ctx.manufacturer")]
pub enum ManufacturerMessage {
  #[marshal(tag = "ni::MANUFACTURER_NI")]
  Ni(
    #[marshal(forward_ctx)]
    ni::NiDeviceMessage
  ),
  #[marshal(tag = "MANUFACTURER_GRAPPLE")]
  Grapple(
    #[marshal(forward_ctx)]
    GrappleDeviceMessage
  )
}
