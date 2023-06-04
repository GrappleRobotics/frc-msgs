#![cfg_attr(not(feature="std"), no_std)]

extern crate alloc;

pub mod grapple;
pub use deku;

use alloc::{format, vec::Vec};
use deku::prelude::*;
use grapple::GrappleDeviceMessage;

pub const DEVICE_ID_BROADCAST: u8 = 0x3F;

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Message {
  #[deku(update = "self.msg.device_type()")]
  pub device_type: u8,
  #[deku(update = "self.msg.deku_id()?")]
  pub manufacturer: u8,
  #[deku(update = "self.msg.api().0")]
  pub api_class: u8,
  #[deku(update = "self.msg.api().1")]
  pub api_index: u8,

  pub device_id: u8,

  #[deku(ctx = "*device_type, *manufacturer, *api_class, *api_index")]
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

    newmsg.update().unwrap();

    newmsg
  }
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[deku(ctx = "device_type: u8, manufacturer: u8, api_class: u8, api_index: u8", id = "manufacturer")]
pub enum ManufacturerMessage {
  #[deku(id = "6")]
  Grapple(
    #[deku(ctx = "device_type, api_class, api_index")]
    GrappleDeviceMessage
  )
}

impl ManufacturerMessage {
  pub fn device_type(&self) -> u8 {
    match self {
      ManufacturerMessage::Grapple(grpl) => grpl.device_type(),
    }
  }

  pub fn api(&self) -> (u8, u8) {
    match self {
      ManufacturerMessage::Grapple(grpl) => grpl.api(),
    }
  }
}