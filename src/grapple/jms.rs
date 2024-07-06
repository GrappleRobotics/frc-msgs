use binmarshal::{AsymmetricCow, BitSpecification, Demarshal, Marshal, MarshalUpdate, Payload, Proxy};
use bounded_static::ToStatic;

use super::GrappleMessageId;

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
pub enum JMSRole {
  #[marshal(tag = "0")]
  ScoringTable,
  #[marshal(tag = "1")]
  Red,
  #[marshal(tag = "2")]
  Blue,
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(C)]
pub struct JMSElectronicsStatus {
  pub role: JMSRole,
  #[marshal(bits = 1)]
  pub io_status: [bool; 8]
}

#[derive(Clone, Debug, PartialEq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_index")]
#[repr(C)]
pub enum JMSMessage<'a> {
  #[marshal(tag = "0")]
  Status(JMSElectronicsStatus),

  #[marshal(tag = "1")]
  SetRole(JMSRole),

  #[marshal(tag = "2")]
  SetDMX(
    #[cfg_attr(feature = "serde", serde(borrow))]
    AsymmetricCow<'a, Payload>
  )
}