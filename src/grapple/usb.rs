use binmarshal::BinMarshal;

use crate::Message;

use crate::can::UnparsedCANMessage;

#[derive(Clone, BinMarshal)]
#[marshal(tag_type = u8)]
pub enum GrappleUSBMessage {
  #[marshal(tag = "0")]
  Message(Message),

  #[marshal(tag = "1")]
  DeviceCheck,   // Used to check that the device connected is indeed a Grapple device

  #[marshal(tag = "2")]
  EncapsulatedCanMessage(u32, UnparsedCANMessage)      // Time (ms), Message. Time is ignored when sending CAN messages
}
