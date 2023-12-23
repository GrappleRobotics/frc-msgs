use binmarshal::BinMarshal;

use crate::{DEVICE_TYPE_BROADCAST, DEVICE_TYPE_FIRMWARE_UPGRADE, Validate, MessageId};
use self::{device_info::GrappleDeviceInfo, firmware::GrappleFirmwareMessage, lasercan::LaserCanMessage, fragments::Fragment};

pub mod device_info;
// pub mod spiderlan;
pub mod lasercan;
pub mod firmware;
pub mod fragments;

pub const MANUFACTURER_GRAPPLE: u8 = 6;
pub const DEVICE_TYPE_DISTANCE_SENSOR: u8 = 6;
pub const DEVICE_TYPE_SPIDERLAN: u8 = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GrappleMessageId {
  pub device_type: u8,
  pub fragment_flag: bool,
  pub ack_flag: bool,
  pub api_class: u8,
  pub api_index: u8,
  pub device_id: u8,
}

impl GrappleMessageId {
  pub fn new(device_id: u8) -> Self {
    Self {
      device_type: 0,
      fragment_flag: false,
      ack_flag: false,
      api_class: 0,
      api_index: 0,
      device_id,
    }
  }
}

impl From<MessageId> for GrappleMessageId {
  fn from(id: MessageId) -> Self {
    Self {
      device_type: id.device_type,
      fragment_flag: id.api_class & 0b100000 != 0,
      ack_flag: id.api_class & 0b010000 != 0,
      api_class: id.api_class & 0b001111,
      api_index: id.api_index,
      device_id: id.device_id,
    }
  }
}

impl From<GrappleMessageId> for MessageId {
  fn from(gid: GrappleMessageId) -> Self {
    Self {
      device_type: gid.device_type,
      manufacturer: MANUFACTURER_GRAPPLE,
      api_class: (gid.api_class & 0b1111) | ((gid.ack_flag as u8) << 4) | ((gid.fragment_flag as u8) << 5),
      api_index: gid.api_index,
      device_id: gid.device_id,
    }
  }
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.fragment_flag")]
pub enum MaybeFragment {
  #[marshal(tag = "true")]
  Fragment(
    #[marshal(ctx = "forward")]
    Fragment
  ),
  #[marshal(tag = "false")]
  Message(
    #[marshal(ctx = "forward")]
    GrappleDeviceMessage
  )
}

impl Validate for MaybeFragment {
  fn validate(&self) -> Result<(), &'static str> {
    match self {
      MaybeFragment::Fragment(_) => Ok(()),
      MaybeFragment::Message(m) => m.validate(),
    }
  }
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.device_type")]
pub enum GrappleDeviceMessage {
  #[marshal(tag = "DEVICE_TYPE_BROADCAST")]
  Broadcast(
    #[marshal(ctx = "forward")]
    GrappleBroadcastMessage
  ),

  #[marshal(tag = "DEVICE_TYPE_FIRMWARE_UPGRADE")]
  FirmwareUpdate(
    #[marshal(ctx = "forward")]
    GrappleFirmwareMessage
  ),

  #[marshal(tag = "DEVICE_TYPE_DISTANCE_SENSOR")]
  DistanceSensor(
    #[marshal(ctx = "forward")]
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
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_class")]
pub enum GrappleBroadcastMessage {
  #[marshal(tag = "0")]
  DeviceInfo(
    #[marshal(ctx = "forward")]
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
