extern crate alloc;
use deku::prelude::*;
use alloc::vec::Vec;
use alloc::format;

use crate::Message;

use crate::can::UnparsedCANMessage;

#[derive(Clone, DekuRead, DekuWrite)]
#[deku(magic = b"GTCP", type = "u8")]
pub enum GrappleTCPMessage {
  #[deku(id = "0")]
  Message(Message),

  #[deku(id = "1")]
  DeviceCheck,   // Used to check that the device connected is indeed a Grapple device

  #[deku(id = "2")]
  EncapsulatedCanMessage(u32, UnparsedCANMessage),      // Time (ms), Message. Time is ignored when sending CAN messages

  #[deku(id = "3")]
  SetCanBridge(bool)
}
