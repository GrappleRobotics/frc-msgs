extern crate alloc;
use deku::prelude::*;
use alloc::format;

// #[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
// #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
// #[deku(ctx = "device_type: u8, api_class: u8, api_index: u8", id = "device_type")]
// pub enum NiDeviceMessage {
//   #[deku(id = "1")]
//   RobotController(
//     #[deku(ctx = "api_class, api_index")]
//     NiRobotControllerMessage
//   )
// }

// impl NiDeviceMessage {
//   pub fn device_type(&self) -> u8 {
//     self.deku_id().unwrap()
//   }

//   pub fn api(&self) -> (u8, u8) {
//     match self {
//       NiDeviceMessage::RobotController(rc) => rc.api(),
//     }
//   }
// }