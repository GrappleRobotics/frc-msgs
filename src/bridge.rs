use binmarshal::{BinMarshal, LengthTaggedVec};

use crate::MessageId;

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct BridgedCANMessage {
  pub id: MessageId,
  pub timestamp: u32,
  pub data: LengthTaggedVec<u8, u8>,
}