use binmarshal::BinMarshal;

use super::device_info::GrappleModelId;

#[derive(Clone, BinMarshal)]
#[marshal(tag_type = u8)]
pub enum GrappleUDPMessage {
  #[marshal(tag = "0")]
  Discover(GrappleModelId)
}
