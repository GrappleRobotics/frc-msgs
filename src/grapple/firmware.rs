use binmarshal::{Marshal, Demarshal, MarshalUpdate, Payload, AsymmetricCow};
use bounded_static::ToStatic;

use crate::Validate;

use super::{GrappleMessageId, errors::GrappleResult};

#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_class")]
pub enum GrappleFirmwareMessage<'a> {
  #[marshal(tag = "0")]
  StartFieldUpgrade { serial: u32 },

  #[marshal(tag = "1")]
  UpdatePart(
    AsymmetricCow<'a, Payload>
  ),

  #[marshal(tag = "2")]
  UpdatePartAck,

  #[marshal(tag = "3")]
  UpdateDone
}

impl<'a> Validate for GrappleFirmwareMessage<'a> {
  fn validate(&self) -> GrappleResult<()> {
    Ok(())
  }
}