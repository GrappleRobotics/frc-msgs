use binmarshal::{Demarshal, MarshalUpdate, Marshal};
use bounded_static::ToStatic;

use crate::{DEVICE_TYPE_BROADCAST, DEVICE_TYPE_FIRMWARE_UPGRADE, Validate, MessageId};
use self::{device_info::GrappleDeviceInfo, firmware::GrappleFirmwareMessage, fragments::Fragment, errors::GrappleResult};

pub mod device_info;
pub mod encapsulation;
// pub mod spiderlan;
#[cfg(feature = "grapple_lasercan")]
pub mod lasercan;
#[cfg(feature = "grapple_mitocandria")]
pub mod mitocandria;
#[cfg(feature = "grapple_flexican")]
pub mod flexican;
pub mod firmware;
pub mod fragments;
pub mod errors;

pub const MANUFACTURER_GRAPPLE: u8 = 6;
pub const DEVICE_TYPE_DISTANCE_SENSOR: u8 = 6;
pub const DEVICE_TYPE_POWER_DISTRIBUTION_MODULE: u8 = 8;
pub const DEVICE_TYPE_IO_BREAKOUT: u8 = 11;
pub const DEVICE_TYPE_SPIDERLAN: u8 = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GrappleMessageId {
  pub device_type: u8,
  pub fragment_flag: bool,
  pub ack_flag: bool,
  pub api_class: u8,
  pub api_index: u8,
  pub device_id: u8,
}

impl GrappleMessageId {
  pub fn new(device_id: u8) -> Self {
    Self {
      device_type: 0,
      fragment_flag: false,
      ack_flag: false,
      api_class: 0,
      api_index: 0,
      device_id,
    }
  }
}

impl From<MessageId> for GrappleMessageId {
  fn from(id: MessageId) -> Self {
    Self {
      device_type: id.device_type,
      fragment_flag: id.api_class & 0b100000 != 0,
      ack_flag: id.api_class & 0b010000 != 0,
      api_class: id.api_class & 0b001111,
      api_index: id.api_index,
      device_id: id.device_id,
    }
  }
}

impl From<GrappleMessageId> for MessageId {
  fn from(gid: GrappleMessageId) -> Self {
    Self {
      device_type: gid.device_type,
      manufacturer: MANUFACTURER_GRAPPLE,
      api_class: (gid.api_class & 0b1111) | ((gid.ack_flag as u8) << 4) | ((gid.fragment_flag as u8) << 5),
      api_index: gid.api_index,
      device_id: gid.device_id,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.fragment_flag", tag_type = "bool")]
pub enum MaybeFragment<'a> {
  #[marshal(tag = "true")]
  Fragment(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    Fragment<'a>
  ),
  #[marshal(tag = "false")]
  Message(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    GrappleDeviceMessage<'a>
  )
}

impl<'a> Validate for MaybeFragment<'a> {
  fn validate(&self) -> GrappleResult<()> {
    match self {
      MaybeFragment::Fragment(_) => Ok(()),
      MaybeFragment::Message(m) => m.validate(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(C)]
pub enum Request<R, A> {
  Ack(A),
  Request(R)
}

// Have to manually implement since the proc macro isn't smart enough to only have a lifetime bound for Demarshal
impl<R: Marshal<()>, A: Marshal<()>> Marshal<GrappleMessageId> for Request<R, A> {
  fn write<W: binmarshal::BitWriter>(&self, writer: &mut W, _ctx: GrappleMessageId) -> Result<(), binmarshal::MarshalError> {
    match self {
      Request::Ack(ack) => ack.write(writer, ()),
      Request::Request(req) => req.write(writer, ()),
    }
  }
}

impl<'dm, R: Demarshal<'dm, ()>, A: Demarshal<'dm, ()>> Demarshal<'dm, GrappleMessageId> for Request<R, A> {
  fn read(view: &mut binmarshal::BitView<'dm>, ctx: GrappleMessageId) -> Result<Self, binmarshal::MarshalError> {
    if ctx.ack_flag {
      Ok(Request::Ack(A::read(view, ())?))
    } else {
      Ok(Request::Request(R::read(view, ())?))
    }
  }
}

impl<R, A> MarshalUpdate<GrappleMessageId> for Request<R, A> {
  fn update(&mut self, ctx: &mut GrappleMessageId) {
    match self {
      Request::Ack(_) => ctx.ack_flag = true,
      Request::Request(_) => ctx.ack_flag = false,
    }
  }
}

impl<R: Validate, A> Validate for Request<R, A> {
  fn validate(&self) -> GrappleResult<()> {
    match self {
      Request::Ack(_) => Ok(()),
      Request::Request(req) => req.validate(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.device_type")]
pub enum GrappleDeviceMessage<'a> {
  #[marshal(tag = "DEVICE_TYPE_BROADCAST")]
  Broadcast(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    GrappleBroadcastMessage<'a>
  ),

  #[marshal(tag = "DEVICE_TYPE_FIRMWARE_UPGRADE")]
  FirmwareUpdate(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    GrappleFirmwareMessage<'a>
  ),

  #[cfg(feature = "grapple_lasercan")]
  #[marshal(tag = "DEVICE_TYPE_DISTANCE_SENSOR")]
  DistanceSensor(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    lasercan::LaserCanMessage<'a>
  ),

  #[cfg(feature = "grapple_mitocandria")]
  #[marshal(tag = "DEVICE_TYPE_POWER_DISTRIBUTION_MODULE")]
  PowerDistributionModule(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    mitocandria::MitocandriaMessage<'a>
  ),

  #[cfg(feature = "grapple_flexican")]
  #[marshal(tag = "DEVICE_TYPE_IO_BREAKOUT")]
  IOBreakout(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    flexican::FlexiCANMessage<'a>
  )
}

impl<'a> Validate for GrappleDeviceMessage<'a> {
  fn validate(&self) -> GrappleResult<()> {
    match self {
      GrappleDeviceMessage::Broadcast(bc) => bc.validate(),
      GrappleDeviceMessage::FirmwareUpdate(fw) => fw.validate(),
      #[cfg(feature = "grapple_lasercan")]
      GrappleDeviceMessage::DistanceSensor(lc) => lc.validate(),
      #[cfg(feature = "grapple_mitocandria")]
      GrappleDeviceMessage::PowerDistributionModule(_) => Ok(()),
      #[cfg(feature = "grapple_flexican")]
      GrappleDeviceMessage::IOBreakout(_) => Ok(())
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_class")]
pub enum GrappleBroadcastMessage<'a> {
  #[marshal(tag = "0")]
  DeviceInfo(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    GrappleDeviceInfo<'a>
  )
}

impl<'a> Validate for GrappleBroadcastMessage<'a> {
  fn validate(&self) -> GrappleResult<()> {
    match self {
      GrappleBroadcastMessage::DeviceInfo(di) => di.validate(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TaggedGrappleMessage<'a> {
  pub device_id: u8,
  #[cfg_attr(feature = "serde", serde(borrow))]
  pub msg: GrappleDeviceMessage<'a>
}

impl<'a> TaggedGrappleMessage<'a> {
  pub fn new(device_id: u8, msg: GrappleDeviceMessage<'a>) -> Self {
    Self { device_id, msg }
  }
}