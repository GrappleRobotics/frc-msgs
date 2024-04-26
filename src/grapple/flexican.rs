use binmarshal::{Demarshal, Marshal, MarshalUpdate};
use bounded_static::ToStatic;

use super::{encapsulation::BridgeMessages, GrappleMessageId};

#[derive(Clone, Debug, PartialEq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_class")]
#[repr(C)]
pub enum FlexiCANMessage<'a> {
  #[marshal(tag = "0")]
  Bridge(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    BridgeMessages<'a>
  ),
}