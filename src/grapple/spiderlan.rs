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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
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
  ),
  #[deku(id = "2")]
  Command(
    #[deku(ctx = "api_index")]
    SpiderLanCommandMessage
  )
}

impl SpiderLanMessage {
  pub fn api(&self) -> (u8, u8) {
    (
      self.deku_id().unwrap(),
      match self {
        SpiderLanMessage::Config(cfg) => cfg.deku_id().unwrap(),
        SpiderLanMessage::Status(sts) => sts.deku_id().unwrap(),
        SpiderLanMessage::Command(cmd) => cmd.deku_id().unwrap()
      }
    )
  }
}

/* CONFIGURATION */
#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_index: u8", id = "api_index")]
pub enum SpiderLanConfigMessage {
  #[deku(id = "0")]
  RequestNetworkConfiguration,

  #[deku(id = "1")]
  NetworkConfiguration(NetworkConfiguration),

  #[deku(id = "2")]
  SetNetworkConfiguration(NetworkConfiguration),

  #[deku(id = "3")]
  SetPinConfiguration(IOPinConfigurationMessage),

  #[deku(id = "4")]
  RequestPinConfigurations,

  #[deku(id = "5")]
  PinConfigurations([IOPinConfiguration; 8]),
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(type = "u8")]
pub enum NetworkConfiguration {
  #[deku(id = "0")]
  Flat(FlatNetworkConfiguration),
  #[deku(id = "1")]
  Vlan(VlanNetworkConfiguration),
  #[deku(id = "2")]
  UplinkFailover(UplinkFailoverConfiguration)
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FlatNetworkConfiguration {
  pub ip: IPConfiguration
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct VlanNetworkConfiguration {
  pub ip: IPConfiguration,
  pub management_vlan: u16,
  pub ports: [PortVlanConfiguration; 4]
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UplinkFailoverConfiguration {
  pub ip: IPConfiguration,
  #[deku(assert = "port_membership.iter().any(|x| *x)")]
  pub port_membership: [bool; 4],   // true if a port is a part of the failover group
  pub ping_upstream_ip: [u8; 4],
  #[deku(assert = "*ping_interval >= 100 && *ping_interval <= 2000")]
  pub ping_interval: u16,   // In ms, 10ms resolution
  #[deku(assert = "*n_fail_before_switch >= 1 && *n_fail_before_switch <= 10")]
  pub n_fail_before_switch: u8,
  #[deku(assert = "*reestablish_cooloff >= 2000 && *reestablish_cooloff <= 30000")]
  pub reestablish_cooloff: u16,          // In ms, 10ms resolution
}

#[derive(Debug, Clone, Copy, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct IPConfiguration {
  pub ip: [u8; 4],
  #[deku(assert = "*prefix <= 32")]
  pub prefix: u8,
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

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct IOPinConfigurationMessage {
  #[deku(bits = 4, assert = "*pin < 8")]
  pub pin: u8,
  pub config: IOPinConfiguration
}

#[derive(Debug, Clone, Copy, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(type="u8", bits=2)]
pub enum DigitalInMode {
  #[deku(id = "0")]
  NoPull,
  #[deku(id = "1")]
  PullUp,
  #[deku(id = "2")]
  PullDown
}

#[derive(Debug, Clone, Copy, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(type="u8", bits=1)]
pub enum DigitalOutMode {
  #[deku(id = "0")]
  PushPull,
  #[deku(id = "1")]
  OpenDrain,
}

#[derive(Debug, Clone, Copy, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(type="u8", bits=2)]
pub enum IOPinConfiguration {
  #[deku(id = "0")]
  Analog,
  #[deku(id = "1")]
  DigitalIn(DigitalInMode),
  #[deku(id = "2")]
  DigitalOut(DigitalOutMode)
}

/* STATUS */
#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_index: u8", id = "api_index")]
pub enum SpiderLanStatusMessage {
  #[deku(id = "0")]
  Network(NetworkStatusFrame),

  #[deku(id = "1")]
  Io(IOStatusFrame)
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct NetworkStatusFrame {
  pub management: PortStatus,
  pub ports: [PortStatus; 4],
  pub specific: NetworkStatusFrameSpecific
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(type = "u8")]
pub enum NetworkStatusFrameSpecific {
  #[deku(id = "0")]
  Flat,
  #[deku(id = "1")]
  Vlan,
  #[deku(id = "2")]
  UplinkFailover {
    active_port: u8,
    upstream_ping_ok: bool,
    n_failed: u8,
    reestablish_cooloff_remaining: u16,
    management_pvid: u16,
    port_pvid: [u16; 4]
  }
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct IOStatusFrame {
  #[deku(bits = 1)]
  pub digital: [bool; 8],
  #[deku(bits = 12)]
  pub analog: [u16; 8]
}

/* COMMAND */
#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_index: u8", id = "api_index")]
pub enum SpiderLanCommandMessage {
  #[deku(id = "0")]
  SetDigitalOut {
    #[deku(bits = 1)]
    set: [bool; 8],
    #[deku(bits = 1)]
    reset: [bool; 8]
  },
  #[deku(id = "1")]
  SetDMX {
    offset: u16,
    #[deku(assert = "(*length + *offset) <= 512", update = "data.len()")]
    length: u16,
    #[deku(count = "length")]
    data: Vec<u8>
  }
}
