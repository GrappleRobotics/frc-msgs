#![cfg_attr(all(not(feature="std"), not(test)), no_std)]

extern crate alloc;

pub mod grapple;
#[cfg(feature = "ni")]
pub mod ni;
pub mod macros;
pub mod bridge;

pub use binmarshal;

use binmarshal::Demarshal;
use binmarshal::Marshal;
use binmarshal::MarshalUpdate;
use bounded_static::ToStatic;
use grapple::MANUFACTURER_GRAPPLE;
use grapple::MaybeFragment;
use grapple::errors::GrappleResult;

pub const DEVICE_TYPE_BROADCAST: u8 = 0x00;
pub const DEVICE_TYPE_FIRMWARE_UPGRADE: u8 = 31;
pub const DEVICE_ID_BROADCAST: u8 = 0x3F;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MessageId {
  pub device_type: u8,
  pub manufacturer: u8,
  pub api_class: u8,
  pub api_index: u8,
  pub device_id: u8,
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

impl From<MessageId> for u32 {
  fn from(v: MessageId) -> Self {
    (v.device_id as u32 & 0b111111)
    | ((v.api_index as u32 & 0b1111) << 6)
    | ((v.api_class as u32 & 0b111111) << (6+4))
    | ((v.manufacturer as u32 & 0b11111111) << (6+4+6))
    | ((v.device_type as u32 & 0b11111) << (6+4+6+8))
  }
}

impl Marshal for MessageId {
  fn write<W: binmarshal::BitWriter>(&self, writer: &mut W, ctx: ()) -> Result<(), binmarshal::MarshalError> {
    Into::<u32>::into(*self).write(writer, ctx)
  }
}

impl<'dm> Demarshal<'dm, ()> for MessageId {
  fn read(view: &mut binmarshal::BitView<'dm>, ctx: ()) -> Result<Self, binmarshal::MarshalError> {
    Ok(Into::<Self>::into(u32::read(view, ctx)?))
  }
}

#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Message<'a> {
  pub id: MessageId,

  #[marshal(
    ctx = "construct",
    ctx_type = MessageId,
    ctx_member(field = "device_type", member = "id.device_type"),
    ctx_member(field = "manufacturer", member = "id.manufacturer"),
    ctx_member(field = "api_class", member = "id.api_class"),
    ctx_member(field = "api_index", member = "id.api_index"),
    ctx_member(field = "device_id", member = "id.device_id"),
  )]
  #[cfg_attr(feature = "serde", serde(borrow))]
  pub msg: ManufacturerMessage<'a>
}

impl<'a> Message<'a> {
  pub fn new(device_id: u8, msg: ManufacturerMessage<'a>) -> Self {
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

impl<'a> Validate for Message<'a> {
  fn validate(&self) -> GrappleResult<()> {
    self.msg.validate()
  }
}

#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = MessageId, tag = "ctx.manufacturer")]
pub enum ManufacturerMessage<'a> {
  #[cfg(feature = "ni")]
  #[marshal(tag = "ni::MANUFACTURER_NI")]
  Ni(
    #[marshal(ctx = "forward")]
    ni::NiDeviceMessage
  ),
  #[marshal(tag = "MANUFACTURER_GRAPPLE")]
  Grapple(
    #[marshal(ctx = "coerce", ctx_type = crate::grapple::GrappleMessageId)]
    #[cfg_attr(feature = "serde", serde(borrow))]
    MaybeFragment<'a>
  )
}

impl<'a> Validate for ManufacturerMessage<'a> {
  fn validate(&self) -> GrappleResult<()> {
    match self {
      #[cfg(feature = "ni")]
      ManufacturerMessage::Ni(_) => Ok(()),
      ManufacturerMessage::Grapple(grpl) => grpl.validate(),
    }
  }
}

pub trait Validate {
  fn validate(&self) -> GrappleResult<()>;
}
