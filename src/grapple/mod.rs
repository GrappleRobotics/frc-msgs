use binmarshal::BinMarshal;

use crate::{DEVICE_TYPE_BROADCAST, DEVICE_TYPE_FIRMWARE_UPGRADE, MessageContext, Validate};
use self::{device_info::GrappleDeviceInfo, firmware::GrappleFirmwareMessage, lasercan::LaserCanMessage};

pub mod device_info;
// pub mod spiderlan;
pub mod lasercan;
pub mod usb;
pub mod tcp;
pub mod udp;
pub mod firmware;

pub const MANUFACTURER_GRAPPLE: u8 = 6;
pub const DEVICE_TYPE_DISTANCE_SENSOR: u8 = 6;
pub const DEVICE_TYPE_SPIDERLAN: u8 = 12;

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageContext, tag = "ctx.device_type")]
pub enum GrappleDeviceMessage {
  #[marshal(tag = "DEVICE_TYPE_BROADCAST")]
  Broadcast(
    #[marshal(forward_ctx)]
    GrappleBroadcastMessage
  ),

  #[marshal(tag = "DEVICE_TYPE_FIRMWARE_UPGRADE")]
  FirmwareUpdate(
    #[marshal(forward_ctx)]
    GrappleFirmwareMessage
  ),

  #[marshal(tag = "DEVICE_TYPE_DISTANCE_SENSOR")]
  DistanceSensor(
    #[marshal(forward_ctx)]
    LaserCanMessage
  ),
}

impl Validate for GrappleDeviceMessage {
  fn validate(&self) -> Result<(), &'static str> {
    match self {
      GrappleDeviceMessage::Broadcast(bc) => bc.validate(),
      GrappleDeviceMessage::FirmwareUpdate(fw) => fw.validate(),
      GrappleDeviceMessage::DistanceSensor(lc) => lc.validate(),
    }
  }
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageContext, tag = "ctx.api_class")]
pub enum GrappleBroadcastMessage {
  #[marshal(tag = "0")]
  DeviceInfo(
    #[marshal(forward_ctx)]
    GrappleDeviceInfo
  )
}

impl Validate for GrappleBroadcastMessage {
  fn validate(&self) -> Result<(), &'static str> {
    match self {
      GrappleBroadcastMessage::DeviceInfo(di) => di.validate(),
    }
  }
}
