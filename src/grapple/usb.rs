extern crate alloc;
use deku::prelude::*;
use alloc::vec::Vec;
use alloc::format;

use crate::Message;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(magic = b"GUSB")]
pub struct GrappleUSBMessage(pub Message);

#[cfg(test)]
mod test {
  use deku::{DekuUpdate, DekuContainerWrite};

  use crate::{grapple::{GrappleDeviceMessage, GrappleBroadcastMessage, device_info::GrappleDeviceInfo}, Message};

  use super::GrappleUSBMessage;

  #[test]
  fn test() {
    let mut msg = GrappleUSBMessage(Message::new(
      0x3F,
      crate::ManufacturerMessage::Grapple(
        GrappleDeviceMessage::Broadcast(GrappleBroadcastMessage::DeviceInfo(GrappleDeviceInfo::EnumerateRequest))
      )
    ));

    msg.update().unwrap();

    println!("{:?}", msg.to_bytes());
  }
}