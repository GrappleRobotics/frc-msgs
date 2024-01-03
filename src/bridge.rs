use binmarshal::{LengthTaggedPayload, Marshal, Demarshal};

use crate::MessageId;

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct BridgedCANMessage<'a> {
  pub id: MessageId,
  pub timestamp: u32,
  #[cfg_attr(feature = "serde", serde(borrow))]
  pub data: LengthTaggedPayload<'a, u8>,
}