use crate::{MessageContext, Validate};
use binmarshal::BinMarshal;

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageContext, tag = "ctx.api_class")]
pub enum GrappleFirmwareMessage {
  #[marshal(tag = "0")]
  StartFieldUpgrade { serial: u32 },
  #[marshal(tag = "1")]
  UpdatePart([u8; 8]),
  #[marshal(tag = "2")]
  UpdatePartAck,
  #[marshal(tag = "3")]
  UpdateDone
}

impl Validate for GrappleFirmwareMessage {
  fn validate(&self) -> Result<(), &'static str> {
    Ok(())
  }
}