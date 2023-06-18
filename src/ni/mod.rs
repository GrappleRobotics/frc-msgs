extern crate alloc;
use deku::prelude::*;
use alloc::format;
use alloc::vec::Vec;

pub const MANUFACTURER_NI: u8 = 0x01;

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "device_type: u8, api_class: u8, api_index: u8", id = "device_type")]
pub enum NiDeviceMessage {
  #[deku(id = "1")]
  RobotController(
    #[deku(ctx = "api_class, api_index")]
    NiRobotControllerMessage
  )
}

impl NiDeviceMessage {
  pub fn device_type(&self) -> u8 {
    self.deku_id().unwrap()
  }

  pub fn api(&self) -> (u8, u8) {
    match self {
      NiDeviceMessage::RobotController(rc) => rc.api(),
    }
  }
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_class: u8, api_index: u8", id = "api_class")]
pub enum NiRobotControllerMessage {
  #[deku(id = "6")]
  Heartbeat(
    #[deku(ctx = "api_index")]
    NiRioHeartbeat
  )
}

impl NiRobotControllerMessage {
  pub fn api(&self) -> (u8, u8) {
    (
      self.deku_id().unwrap(),
      match self {
        NiRobotControllerMessage::Heartbeat(hb) => hb.deku_id().unwrap(),
      }
    )
  }
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_index: u8", id = "api_index")]
pub enum NiRioHeartbeat {
  #[deku(id = "1")]
  Hearbeat(NiRioHearbeat1)
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NiRioHearbeat1 {
  pub reserved1: u8,
  pub reserved2: u8,
  pub reserved3: u8,
  pub reserved4: u8,
  #[deku(bits = 3)]
  pub reserved5 : u8,
  #[deku(bits = 1)]
  pub watchdog_enabled: bool,
  #[deku(bits = 1)]
  pub test: bool,
  #[deku(bits = 1)]
  pub autonomous: bool,
  #[deku(bits = 1)]
  pub enabled: bool,
  #[deku(bits = 1)]
  pub red_alliance: bool,
  pub reserved6: u8,
  pub reserved7: u8,
  pub reserved8: u8,
}