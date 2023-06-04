extern crate alloc;
use deku::prelude::*;
use alloc::format;

use self::device_info::GrappleDeviceInfo;

pub mod device_info;
pub mod usb;

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "device_type: u8, api_class: u8, api_index: u8", id = "device_type")]
pub enum GrappleDeviceMessage {
  #[deku(id = "0")]
  Broadcast(
    #[deku(ctx = "api_class, api_index")]
    GrappleBroadcastMessage
  ),

  #[deku(id = "31")]
  FirmwareUpdate,

  #[deku(id = "6")]
  DistanceSensor,

  #[deku(id = "12")]
  EthernetSwitch,     // TODO: Submit a request to FIRST for this once we go public. This ID may change.
}

impl GrappleDeviceMessage {
  pub fn device_type(&self) -> u8 {
    self.deku_id().unwrap()
  }

  pub fn api(&self) -> (u8, u8) {
    match self {
      GrappleDeviceMessage::Broadcast(bcast) => bcast.api(),
      GrappleDeviceMessage::FirmwareUpdate => unreachable!(),
      GrappleDeviceMessage::DistanceSensor => unreachable!(),
      GrappleDeviceMessage::EthernetSwitch => unreachable!(),
    }
  }
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_class: u8, api_index: u8", id = "api_class")]
pub enum GrappleBroadcastMessage {
  #[deku(id = "0")]
  DeviceInfo(
    #[deku(ctx = "api_index")]
    GrappleDeviceInfo
  )
}

impl GrappleBroadcastMessage {
  pub fn api(&self) -> (u8, u8) {
    (
      self.deku_id().unwrap(),
      match self {
        GrappleBroadcastMessage::DeviceInfo(di) => di.deku_id().unwrap(),
      }
    )
  }
}