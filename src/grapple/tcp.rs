extern crate alloc;
use deku::prelude::*;
use alloc::vec::Vec;
use alloc::format;

use crate::Message;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(magic = b"GTCP", type = "u8")]
pub enum GrappleTCPMessage {
  #[deku(id = "0")]
  Encapsulated(Message),

  #[deku(id = "1")]
  DeviceCheck   // Used to check that the device connected is indeed a Grapple device
}
