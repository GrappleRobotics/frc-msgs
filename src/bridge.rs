use binmarshal::{LengthTaggedPayload, Marshal, Demarshal, AsymmetricCow};
use bounded_static::ToStatic;

use crate::MessageId;

#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct BridgedCANMessage<'a> {
  pub id: MessageId,
  pub timestamp: u32,
  pub data: AsymmetricCow<'a, LengthTaggedPayload<u8>>
}
