use crate::MessageContext;
use binmarshal::BinMarshal;

#[derive(Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct LaserCanRoi {
  #[marshal(bits = 4)]
  pub x: u8,
  #[marshal(bits = 4)]
  pub y: u8,
  #[marshal(bits = 4)]
  pub w: u8,
  #[marshal(bits = 4)]
  pub h: u8
}

#[derive(Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageContext, tag = "ctx.api_class")]
pub enum LaserCanMessage {
  #[marshal(tag = "0")]
  Status {
    status: u8,
    distance_mm: u16,
    ambient: u16,
    #[marshal(bits = 1)]
    long: bool,
    #[marshal(bits = 7)]
    budget_ms: u8,
    roi: LaserCanRoi
  },
  #[marshal(tag = "1")]
  SetRange {
    long: bool
  },
  #[marshal(tag = "2")]
  SetRoi {
    roi: LaserCanRoi
  },
  #[marshal(tag = "3")]
  SetTimingBudget {
    budget: u8
  },
}
