extern crate alloc;
use deku::prelude::*;
use alloc::{format, vec::Vec};

#[derive(Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct LaserCanRoi {
  #[deku(bits = 4)]
  pub x: u8,
  #[deku(bits = 4)]
  pub y: u8,
  #[deku(bits = 4)]
  pub w: u8,
  #[deku(bits = 4)]
  pub h: u8
}

#[derive(Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_class: u8, api_index: u8", id = "api_class")]
pub enum LaserCanMessage {
  #[deku(id = "0")]
  Status {
    status: u8,
    distance_mm: u16,
    ambient: u16,
    #[deku(bits = 1)]
    long: bool,
    #[deku(bits = 7)]
    budget_ms: u8,
    roi: LaserCanRoi
  },
  #[deku(id = "1")]
  SetRange {
    long: bool
  },
  #[deku(id = "2")]
  SetRoi {
    roi: LaserCanRoi
  },
  #[deku(id = "3")]
  SetTimingBudget {
    budget: u8
  },
}

impl LaserCanMessage {
  pub fn api(&self) -> (u8, u8) {
    (
      self.deku_id().unwrap(),
      0
    )
  }
}