use crate::{MessageContext, Validate};
use binmarshal::{BinMarshal, Proxy, BitSpecification};
use core::ops::{Deref, DerefMut};

#[derive(Proxy)]
#[repr(transparent)]
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
#[repr(C)]
pub struct LaserCanRoi {
  pub x: LaserCanRoiU4,
  pub y: LaserCanRoiU4,
  pub w: LaserCanRoiU4,
  pub h: LaserCanRoiU4
}

impl Validate for LaserCanRoi {
  fn validate(&self) -> Result<(), &'static str> {
    if self.w.0 % 2 != 0 || self.h.0 % 2 != 0 { Err("LaserCanRoi: width and height must be even!")? };
    let hw = self.w.0 / 2;
    let hh = self.h.0 / 2;

    let xmin = self.x.0 as i16 - hw as i16;
    let xmax = self.x.0 as i16 + hw as i16;
    let ymin = self.y.0 as i16 - hh as i16;
    let ymax = self.y.0 as i16 + hh as i16;

    if xmin < 0 || xmax > 16 || ymin < 0 || ymax > 16 { Err("LaserCanRoi: out of bounds!")? }

    Ok(())
  }
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(C)]
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
#[repr(C)]
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
  #[marshal(tag = "4")]
  SetLedThreshold {
    distance_mm: u16    // 0 for off
  }
}

impl Validate for LaserCanMessage {
  fn validate(&self) -> Result<(), &'static str> {
    match self {
      LaserCanMessage::Status(_) => Ok(()),
      LaserCanMessage::SetRange { .. } => Ok(()),
      LaserCanMessage::SetRoi { roi } => roi.validate(),
      LaserCanMessage::SetTimingBudget { budget } => match budget {
        20 => Ok(()),
        33 => Ok(()),
        50 => Ok(()),
        100 => Ok(()),
        _ => Err("LaserCanMessage: invalid timing budget!")
      },
      LaserCanMessage::SetLedThreshold { distance_mm } => match distance_mm {
        21..4000 => Ok(()),
        _ => Err("LaserCanMessage: invalid LED threshold. Must be under >20, <4000mm.")
      }
    }
  }
}
