extern crate alloc;
use deku::prelude::*;
use alloc::vec::Vec;
use alloc::format;

use crate::Message;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(magic = b"GUSB", type = "u8")]
pub enum GrappleUSBMessage {
  #[deku(id = "0")]
  Encapsulated(Message),

  #[deku(id = "1")]
  DeviceCheck   // Used to check that the device connected is indeed a Grapple device
}

#[cfg(test)]
mod test {
  use deku::{DekuUpdate, DekuContainerWrite};

  use crate::{grapple::{GrappleDeviceMessage, GrappleBroadcastMessage, device_info::GrappleDeviceInfo}, Message};

  use super::GrappleUSBMessage;

  #[test]
  fn test() {
    let mut msg = GrappleUSBMessage::Encapsulated(Message::new(
      0x3F,
      crate::ManufacturerMessage::Grapple(
        GrappleDeviceMessage::Broadcast(GrappleBroadcastMessage::DeviceInfo(GrappleDeviceInfo::EnumerateRequest))
      )
    ));

    msg.update().unwrap();

    println!("{:?}", msg.to_bytes());
  }
}