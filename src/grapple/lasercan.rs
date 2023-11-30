use crate::MessageContext;
use binmarshal::{BinMarshal, Proxy, BitSpecification};
use core::ops::{Deref, DerefMut};

#[derive(Proxy)]
pub struct LaserCanRoiU4(pub u8);

impl BinMarshal<()> for LaserCanRoiU4 {
  type Context = ();

  fn write<W: binmarshal::rw::BitWriter>(self, writer: &mut W, ctx: ()) -> bool {
    (self.0 - 1).write(writer, BitSpecification::<4>)
  }

  fn read(view: &mut binmarshal::rw::BitView<'_>, ctx: ()) -> Option<Self> {
    u8::read(view, BitSpecification::<4>).map(|x| Self(x + 1))
  }

  fn update<'a>(&'a mut self, _ctx: <() as binmarshal::BinmarshalContext>::MutableComplement<'a>) { }
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct LaserCanRoi {
  pub x: LaserCanRoiU4,
  pub y: LaserCanRoiU4,
  pub w: LaserCanRoiU4,
  pub h: LaserCanRoiU4
}


#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct LaserCanStatusFrame {
  pub status: u8,
  pub distance_mm: u16,
  pub ambient: u16,
  #[marshal(bits = 1)]
  pub long: bool,
  #[marshal(bits = 7)]
  pub budget_ms: u8,
  pub roi: LaserCanRoi
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageContext, tag = "ctx.api_class")]
pub enum LaserCanMessage {
  #[marshal(tag = "0")]
  Status(LaserCanStatusFrame),
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
