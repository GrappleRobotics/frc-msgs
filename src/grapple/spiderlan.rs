extern crate alloc;
use deku::prelude::*;
use alloc::{format, vec::Vec};

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(type = "u8")]
pub enum PortDuplexStatus {
  #[deku(id = "0")]
  Half,
  #[deku(id = "1")]
  Full,
  #[deku(id = "2")]
  Unknown
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(type = "u8")]
pub enum PortStatus {
  #[deku(id = "0")]
  NoLink,
  #[deku(id = "1")]
  AutonegotiationInProgress,
  #[deku(id = "2")]
  LinkUp {
    speed: u16,
    duplex: PortDuplexStatus,
  }
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_class: u8, api_index: u8", id = "api_class")]
pub enum SpiderLanMessage {
  #[deku(id = "0")]
  Config(
    #[deku(ctx = "api_index")]
    SpiderLanConfigMessage
  ),
  #[deku(id = "1")]
  Status(
    #[deku(ctx = "api_index")]
    SpiderLanStatusMessage
  )
}

impl SpiderLanMessage {
  pub fn api(&self) -> (u8, u8) {
    (
      self.deku_id().unwrap(),
      match self {
        SpiderLanMessage::Config(cfg) => cfg.deku_id().unwrap(),
        SpiderLanMessage::Status(sts) => sts.deku_id().unwrap()
      }
    )
  }
}

/* CONFIGURATION */

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_index: u8", id = "api_index")]
pub enum SpiderLanConfigMessage {
  #[deku(id = "0")]
  RequestPortsConfiguration,

  #[deku(id = "1")]
  PortsConfiguration(PortsConfiguration),

  #[deku(id = "2")]
  SetPortsConfiguration(PortsConfiguration)
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PortsConfiguration {
  pub vlans_enabled: bool,
  pub vlans: [PortVlanConfiguration; 5]
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PortVlanConfiguration {
  pub native_vlan: u16,

  #[deku(assert = "*num_tagged <= 16", update = "self.tagged_vlans.len()")]
  num_tagged: u8,
  #[deku(count = "num_tagged")]
  pub tagged_vlans: Vec<u16>
}

impl Default for PortVlanConfiguration {
  fn default() -> Self {
    Self { native_vlan: 1, num_tagged: 0, tagged_vlans: alloc::vec![] }
  }
}


/* STATUS */
#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_index: u8", id = "api_index")]
pub enum SpiderLanStatusMessage {
  #[deku(id = "0")]
  PortStatusFrame(SpiderLanPortStatusFrame),
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SpiderLanPortStatusFrame {
  pub ports: [PortStatus; 5]
}
