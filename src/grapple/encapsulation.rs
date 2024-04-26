use binmarshal::{AsymmetricCow, Demarshal, LengthTaggedPayload, LengthTaggedVec, Marshal, MarshalUpdate};
use bounded_static::ToStatic;

use crate::MessageId;

use super::{errors::GrappleResult, GrappleMessageId, Request};

#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct EncapsulatedMesssage<'a> {
  pub channel: AsymmetricCow<'a, str>,
  pub timestamp: u32,
  pub id: MessageId,
  pub data: AsymmetricCow<'a, LengthTaggedPayload<u8>>
}

#[derive(Clone, Debug, PartialEq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_index")]
#[repr(C)]
pub enum BridgeMessages<'a> {
  #[marshal(tag = "0")]
  GetChannelName(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    Request<u8, GrappleResult<'a, AsymmetricCow<'a, str>>>
  ),
  #[marshal(tag = "1")]
  StartBridge(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    Request<AsymmetricCow<'a, str>, GrappleResult<'a, ()>>
  ),
  #[marshal(tag = "2")]
  StopBridge(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    Request<AsymmetricCow<'a, str>, GrappleResult<'a, ()>>
  ),
  #[marshal(tag = "3")]
  BridgeMessage(EncapsulatedMesssage<'a>)
}