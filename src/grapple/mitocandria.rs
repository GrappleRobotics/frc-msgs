// This is all temporary until we convert to FEVAMS prior to release.

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
pub enum ChannelStatus {
  #[marshal(tag = "0")]
  Switchable {
    enabled: bool,
    current: u16
  },
  #[marshal(tag = "1")]
  NonSwitchable {
    current: u16
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "pyo3", pyclass)]
#[repr(C)]
pub struct StatusFrame {
  pub channels: [ChannelStatus; 5],
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "pyo3", pyclass)]
#[repr(C)]
pub struct SwitchableChannelRequest {
  pub channel: u8,
  pub enabled: bool
}

#[derive(Clone, Debug, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_class")]
#[repr(C)]
pub enum PowerfulPandaMessage<'a> {
  #[marshal(tag = "0")]
  StatusFrame(StatusFrame),
  #[marshal(tag = "1")]
  SetSwitchableChannel(
    #[marshal(ctx = "forward")]
    #[cfg_attr(feature = "serde", serde(borrow))]
    Request<SwitchableChannelRequest, GrappleResult<'a, ()>>
  )
}