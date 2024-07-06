use binmarshal::{AsymmetricCow, BitSpecification, Demarshal, Marshal, MarshalUpdate, Payload, Proxy};
use bounded_static::ToStatic;

use super::GrappleMessageId;

#[derive(Clone, Debug, PartialEq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_class")]
#[repr(C)]
pub enum MiscMessage<'a> {
  #[marshal(tag = "0")]
  MiscMessage(
    #[cfg_attr(feature = "serde", serde(borrow))]
    AsymmetricCow<'a, Payload>
  ),

  #[cfg(feature = "grapple_jms")]
  #[marshal(tag = "1")]
  JMS(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    crate::grapple::jms::JMSMessage<'a>
  )
}