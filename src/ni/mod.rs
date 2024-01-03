extern crate alloc;
use binmarshal::{Demarshal, Marshal, MarshalUpdate};

use crate::MessageId;

pub const MANUFACTURER_NI: u8 = 0x01;

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageId, tag = "ctx.device_type")]
pub enum NiDeviceMessage {
  #[marshal(tag = "1")]
  RobotController(
    #[marshal(ctx = "forward")]
    NiRobotControllerMessage
  )
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageId, tag = "ctx.api_class")]
pub enum NiRobotControllerMessage {
  #[marshal(tag = "6")]
  Heartbeat(
    #[marshal(ctx = "forward")]
    NiRioHeartbeat
  )
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageId, tag = "ctx.api_index")]
pub enum NiRioHeartbeat {
  #[marshal(tag = "1")]
  Hearbeat(NiRioHearbeat1)
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NiRioHearbeat1 {
  pub reserved1: u8,
  pub reserved2: u8,
  pub reserved3: u8,
  pub reserved4: u8,
  #[marshal(bits = 3)]
  pub reserved5 : u8,
  #[marshal(bits = 1)]
  pub watchdog_enabled: bool,
  #[marshal(bits = 1)]
  pub test: bool,
  #[marshal(bits = 1)]
  pub autonomous: bool,
  #[marshal(bits = 1)]
  pub enabled: bool,
  #[marshal(bits = 1)]
  pub red_alliance: bool,
  pub reserved6: u8,
  pub reserved7: u8,
  pub reserved8: u8,
}