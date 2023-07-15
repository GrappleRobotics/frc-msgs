extern crate alloc;
use deku::prelude::*;
use alloc::format;

use crate::{DEVICE_TYPE_BROADCAST, DEVICE_TYPE_FIRMWARE_UPGRADE};
use self::{device_info::GrappleDeviceInfo, spiderlan::SpiderLanMessage, firmware::GrappleFirmwareMessage};

pub mod device_info;
pub mod spiderlan;
pub mod usb;
pub mod tcp;
pub mod udp;
pub mod firmware;

pub const MANUFACTURER_GRAPPLE: u8 = 6;
pub const DEVICE_TYPE_SPIDERLAN: u8 = 12;

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "device_type: u8, api_class: u8, api_index: u8", id = "device_type")]
pub enum GrappleDeviceMessage {
  #[deku(id = "DEVICE_TYPE_BROADCAST")]
  Broadcast(
    #[deku(ctx = "api_class, api_index")]
    GrappleBroadcastMessage
  ),

  #[deku(id = "DEVICE_TYPE_FIRMWARE_UPGRADE")]
  FirmwareUpdate(
    #[deku(ctx = "api_class, api_index")]
    GrappleFirmwareMessage
  ),

  #[deku(id = "6")]
  DistanceSensor,

  // TODO: Submit a request to FIRST for this once we go public. This ID may change.
  #[deku(id = "DEVICE_TYPE_SPIDERLAN")]
  EthernetSwitch(
    #[deku(ctx = "api_class, api_index")]
    SpiderLanMessage
  ),
}

impl GrappleDeviceMessage {
  pub fn device_type(&self) -> u8 {
    self.deku_id().unwrap()
  }

  pub fn api(&self) -> (u8, u8) {
    match self {
      GrappleDeviceMessage::Broadcast(bcast) => bcast.api(),
      GrappleDeviceMessage::FirmwareUpdate(fware) => fware.api(),
      GrappleDeviceMessage::DistanceSensor => unreachable!(),
      GrappleDeviceMessage::EthernetSwitch(switch) => switch.api(),
    }
  }
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
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