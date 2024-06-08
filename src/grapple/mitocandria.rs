use binmarshal::{Demarshal, Marshal, MarshalUpdate};
use bounded_static::ToStatic;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use super::{errors::GrappleResult, GrappleMessageId, Request};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
#[repr(C)]
pub enum MitocandriaChannelStatus {
  #[marshal(tag = "0")]
  Switchable {
    enabled: bool,
    current: u16
  },
  #[marshal(tag = "1")]
  NonSwitchable {
    current: u16,
  },
  #[marshal(tag = "2")]
  Adjustable {
    enabled: bool,
    voltage: u16,
    voltage_setpoint: u16,
    current: u16,
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "pyo3", pyclass)]
#[repr(C)]
pub struct MitocandriaStatusFrame {
  pub channels: [MitocandriaChannelStatus; 5],
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "pyo3", pyclass)]
#[repr(C)]
pub struct MitocandriaSwitchableChannelRequest {
  pub channel: u8,
  pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "pyo3", pyclass)]
#[repr(C)]
pub struct MitocandriaAdjustableChannelRequest {
  pub channel: u8,
  pub enabled: bool,
  pub voltage: u16
}

#[derive(Clone, Debug, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_index")]
#[repr(C)]
pub enum MitocandriaChannelRequest<'a> {
  #[marshal(tag = "0")]
  SetSwitchableChannel(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    Request<MitocandriaSwitchableChannelRequest, GrappleResult<'a, ()>>
  ),
  #[marshal(tag = "1")]
  SetAdjustableChannel(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    Request<MitocandriaAdjustableChannelRequest, GrappleResult<'a, ()>>
  )
}

#[derive(Clone, Debug, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_class")]
#[repr(C)]
pub enum MitocandriaMessage<'a> {
  #[marshal(tag = "0")]
  StatusFrame(MitocandriaStatusFrame),
  #[marshal(tag = "1")]
  ChannelRequest(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    MitocandriaChannelRequest<'a>
  )
}