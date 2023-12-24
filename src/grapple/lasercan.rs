use crate::Validate;
use binmarshal::{BinMarshal, Proxy, BitSpecification};
use core::ops::{Deref, DerefMut};

use super::{GrappleMessageId, Request, errors::{GrappleResult, GrappleError}};

#[derive(Proxy)]
#[repr(transparent)]
pub struct LaserCanRoiU4(pub u8);

impl BinMarshal<()> for LaserCanRoiU4 {
  type Context = ();

  fn write<W: binmarshal::rw::BitWriter>(&self, writer: &mut W, _ctx: ()) -> bool {
    (self.0 - 1).write(writer, BitSpecification::<4>)
  }

  fn read(view: &mut binmarshal::rw::BitView<'_>, _ctx: ()) -> Option<Self> {
    u8::read(view, BitSpecification::<4>).map(|x| Self(x + 1))
  }

  fn update<'a>(&'a mut self, _ctx: &mut ()) { }
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
  fn validate(&self) -> GrappleResult<()> {
    if self.w.0 % 2 != 0 || self.h.0 % 2 != 0 {
      Err(GrappleError::ParameterOutOfBounds("LaserCanRoi: width and height must be even".to_owned()))?;
    };
    let hw = self.w.0 / 2;
    let hh = self.h.0 / 2;

    let xmin = self.x.0 as i16 - hw as i16;
    let xmax = self.x.0 as i16 + hw as i16;
    let ymin = self.y.0 as i16 - hh as i16;
    let ymax = self.y.0 as i16 + hh as i16;

    if xmin < 0 || xmax > 16 || ymin < 0 || ymax > 16 {
      Err(GrappleError::ParameterOutOfBounds("LaserCanRoi: out of bounds".to_owned()))?;
    }

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
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_class")]
#[repr(C)]
pub enum LaserCanMessage {
  #[marshal(tag = "0")]
  Status(LaserCanStatusFrame),
  #[marshal(tag = "1")]
  SetRange(
    #[marshal(ctx = "forward")]
    Request<bool, GrappleResult<()>>
  ),
  #[marshal(tag = "2")]
  SetRoi(
    #[marshal(ctx = "forward")]
    Request<LaserCanRoi, GrappleResult<()>>
  ),
  #[marshal(tag = "3")]
  SetTimingBudget(
    #[marshal(ctx = "forward")]
    Request<u8, GrappleResult<()>>
  ),
  #[marshal(tag = "4")]
  SetLedThreshold(
    #[marshal(ctx = "forward")]
    Request<u16, GrappleResult<()>>    // 0 for off
  )
}

impl Validate for LaserCanMessage {
  fn validate(&self) -> GrappleResult<()> {
    match self {
      LaserCanMessage::Status(_) => Ok(()),
      LaserCanMessage::SetRange { .. } => Ok(()),
      LaserCanMessage::SetRoi(roi) => roi.validate(),
      LaserCanMessage::SetTimingBudget(budget) => match budget {
        Request::Ack(_) => Ok(()),
        Request::Request(20) => Ok(()),
        Request::Request(33) => Ok(()),
        Request::Request(50) => Ok(()),
        Request::Request(100) => Ok(()),
        _ => Err(GrappleError::ParameterOutOfBounds("Invalid Timing Budget".to_string()))
      },
      LaserCanMessage::SetLedThreshold(distance_mm) => match distance_mm {
        Request::Ack(_) => Ok(()),
        Request::Request(21..=4000) => Ok(()),
        Request::Request(0) => Ok(()),      // Turned off
        _ => Err(GrappleError::ParameterOutOfBounds("Invalid LED threshold. Must be under >20, <4000mm.".to_string()))
      }
    }
  }
}
