#![cfg_attr(all(not(feature="std"), not(test)), no_std)]

extern crate alloc;

pub mod grapple;
pub mod ni;

pub use binmarshal;

use binmarshal::BinMarshal;
use grapple::MANUFACTURER_GRAPPLE;
use grapple::MaybeFragment;

pub const DEVICE_TYPE_BROADCAST: u8 = 0x00;
pub const DEVICE_TYPE_FIRMWARE_UPGRADE: u8 = 31;
pub const DEVICE_ID_BROADCAST: u8 = 0x3F;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MessageId {
  device_type: u8,
  manufacturer: u8,
  api_class: u8,
  api_index: u8,
  #[allow(dead_code)]
  device_id: u8,
}

impl From<u32> for MessageId {
  fn from(value: u32) -> Self {
    Self {
      device_type: ((value >> 6+4+6+8) & 0b11111) as u8,
      manufacturer: ((value >> 6+4+6) & 0b11111111) as u8,
      api_class: ((value >> 6+4) & 0b111111) as u8,
      api_index: ((value >> 6) & 0b1111) as u8,
      device_id: (value & 0b111111) as u8
    }
  }
}

impl Into<u32> for MessageId {
  fn into(self) -> u32 {
    (self.device_id as u32 & 0b111111)
    | ((self.api_index as u32 & 0b1111) << 6)
    | ((self.api_class as u32 & 0b111111) << (6+4))
    | ((self.manufacturer as u32 & 0b11111111) << (6+4+6))
    | ((self.device_type as u32 & 0b11111) << (6+4+6+8))
  }
}

impl BinMarshal<()> for MessageId {
  type Context = ();

  fn write<W: binmarshal::BitWriter>(self, writer: &mut W, ctx: ()) -> bool {
    Into::<u32>::into(self).write(writer, ctx)
  }

  fn read(view: &mut binmarshal::BitView<'_>, ctx: ()) -> Option<Self> {
    Some(Into::<Self>::into(u32::read(view, ctx)?))
  }

  fn update(&mut self, _ctx: &mut ()) { }
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Message {
  pub id: MessageId,
  #[marshal(
    ctx = "construct",
    ctx_member(field = "device_type", member = "id.device_type"),
    ctx_member(field = "manufacturer", member = "id.manufacturer"),
    ctx_member(field = "api_class", member = "id.api_class"),
    ctx_member(field = "api_index", member = "id.api_index"),
    ctx_member(field = "device_id", member = "id.device_id"),
  )]
  pub msg: ManufacturerMessage
}

impl Message {
  pub fn new(device_id: u8, msg: ManufacturerMessage) -> Self {
    let mut newmsg = Self {
      id: MessageId {
        device_type: 0,
        manufacturer: 0,
        api_class: 0,
        api_index: 0,
        device_id,
      },
      msg,
    };

    newmsg.update(&mut ());

    newmsg
  }
}

impl Validate for Message {
  fn validate(&self) -> Result<(), &'static str> {
    self.msg.validate()
  }
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageId, tag = "ctx.manufacturer")]
pub enum ManufacturerMessage {
  #[marshal(tag = "ni::MANUFACTURER_NI")]
  Ni(
    #[marshal(ctx = "forward")]
    ni::NiDeviceMessage
  ),
  #[marshal(tag = "MANUFACTURER_GRAPPLE")]
  Grapple(
    #[marshal(ctx = "coerce")]
    MaybeFragment
  )
}

impl Validate for ManufacturerMessage {
  fn validate(&self) -> Result<(), &'static str> {
    match self {
      ManufacturerMessage::Ni(_) => Ok(()),
      ManufacturerMessage::Grapple(grpl) => grpl.validate(),
    }
  }
}

pub trait Validate {
  fn validate(&self) -> Result<(), &'static str>;
}