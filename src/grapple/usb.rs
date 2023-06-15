extern crate alloc;
use deku::prelude::*;
use alloc::vec::Vec;
use alloc::format;

use crate::Message;

use crate::can::UnparsedCANMessage;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(magic = b"GUSB", type = "u8")]
pub enum GrappleUSBMessage {
  #[deku(id = "0")]
  Message(Message),

  #[deku(id = "1")]
  DeviceCheck,   // Used to check that the device connected is indeed a Grapple device

  #[deku(id = "2")]
  EncapsulatedCanMessage(UnparsedCANMessage)
}
