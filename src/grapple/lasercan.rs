use crate::Validate;
use binmarshal::{BinMarshal, Proxy, BitSpecification};
use core::ops::{Deref, DerefMut};

use super::{GrappleMessageId, Request, errors::{GrappleResult, GrappleError, CowStr}};

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
      Err(GrappleError::ParameterOutOfBounds(CowStr::Borrowed("LaserCanRoi: width and height must be even")))?;
    };
    let hw = self.w.0 / 2;
    let hh = self.h.0 / 2;

    let xmin = self.x.0 as i16 - hw as i16;
    let xmax = self.x.0 as i16 + hw as i16;
    let ymin = self.y.0 as i16 - hh as i16;
    let ymax = self.y.0 as i16 + hh as i16;

    if xmin < 0 || xmax > 16 || ymin < 0 || ymax > 16 {
      Err(GrappleError::ParameterOutOfBounds(CowStr::Borrowed("LaserCanRoi: out of bounds")))?;
    }

    Ok(())
  }
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8", tag_bits = 7)]
#[repr(u8)]
pub enum LaserCanTimingBudget {
  #[marshal(tag = "20")]
  TB20ms = 20,
  #[marshal(tag = "33")]
  TB33ms = 33,
  #[marshal(tag = "50")]
  TB50ms = 50,
  #[marshal(tag = "100")]
  TB100ms = 100,
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "bool", tag_bits = 1)]
#[repr(u8)]
pub enum LaserCanRangingMode {
  #[marshal(tag = "false")]
  Short,
  #[marshal(tag = "true")]
  Long
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(C)]
pub struct LaserCanMeasurement {
  // This struct should be 8 bytes or less to fit in a single status frame
  pub status: u8,
  pub distance_mm: u16,
  pub ambient: u16,
  pub long: LaserCanRangingMode,
  pub budget_ms: LaserCanTimingBudget,
  pub roi: LaserCanRoi
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_class")]
#[repr(C)]
pub enum LaserCanMessage {
  #[marshal(tag = "0")]
  Measurement(LaserCanMeasurement),
  #[marshal(tag = "1")]
  SetRange(
    #[marshal(ctx = "forward")]
    Request<LaserCanRangingMode, GrappleResult<()>>
  ),
  #[marshal(tag = "2")]
  SetRoi(
    #[marshal(ctx = "forward")]
    Request<LaserCanRoi, GrappleResult<()>>
  ),
  #[marshal(tag = "3")]
  SetTimingBudget(
    #[marshal(ctx = "forward")]
    Request<LaserCanTimingBudget, GrappleResult<()>>
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
      LaserCanMessage::Measurement(_) => Ok(()),
      LaserCanMessage::SetRange(..) => Ok(()),
      LaserCanMessage::SetRoi(roi) => roi.validate(),
      LaserCanMessage::SetTimingBudget(..) => Ok(()),
      LaserCanMessage::SetLedThreshold(distance_mm) => match distance_mm {
        Request::Ack(_) => Ok(()),
        Request::Request(21..=4000) => Ok(()),
        Request::Request(0) => Ok(()),      // Turned off
        _ => Err(GrappleError::ParameterOutOfBounds(CowStr::Borrowed("Invalid LED threshold. Must be under >20, <4000mm.")))
      }
    }
  }
}
