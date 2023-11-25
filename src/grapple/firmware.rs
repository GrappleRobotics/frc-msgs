use crate::MessageContext;
use binmarshal::{BinMarshal, LengthTaggedVec};

#[derive(Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageContext, tag = "ctx.api_class")]
pub enum GrappleFirmwareMessage {
  #[marshal(tag = "0")]
  StartFieldUpgrade { serial: u32 },
  #[marshal(tag = "1")]
  UpdatePart {
    serial: u32,
    data: LengthTaggedVec<u8, u8>
  },
  #[marshal(tag = "2")]
  UpdatePartAck { serial: u32 },
  #[marshal(tag = "3")]
  UpdateDone { serial: u32 }
}
