extern crate alloc;
use deku::prelude::*;
use alloc::vec::Vec;
use alloc::format;

use super::device_info::GrappleModelId;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(magic = b"GUDP", type = "u8")]
pub enum GrappleUDPMessage {
  #[deku(id = "0")]
  Discover(GrappleModelId)
}
