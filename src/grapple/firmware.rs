use binmarshal::{Marshal, Demarshal, MarshalUpdate, Payload, AsymmetricCow};
use bounded_static::ToStatic;

use crate::Validate;

use super::{errors::GrappleResult, GrappleMessageId, Request};

// This will always fragment on CAN 2.0, but that's ok - it's faster and more resilient to dropped packets
// since we can retry with a defined offset, so if the Ack gets lost that's also ok. 
#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(C)]
pub struct UpdatePartV2Payload<'a> {
  pub offset: u32,
  pub payload: AsymmetricCow<'a, Payload>
}

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
  UpdateDone,

  // This can be automatically detected by trying to perform UpdatePartV2, and if it fails,
  // reverting back to UpdatePart (V1).
  #[marshal(tag = "4")]
  UpdatePartV2(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    Request<UpdatePartV2Payload<'a>, GrappleResult<'a, ()>>
  )
}

impl<'a> Validate for GrappleFirmwareMessage<'a> {
  fn validate(&self) -> GrappleResult<()> {
    Ok(())
  }
}