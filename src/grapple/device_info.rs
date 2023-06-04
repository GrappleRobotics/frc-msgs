extern crate alloc;
use deku::prelude::*;
use alloc::vec::Vec;
use alloc::format;

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(type = "u8")]
pub enum GrappleModelId {
  LaserCan = 0x00,
  SpiderLan = 0x10,
}

#[derive(Debug, Clone, DekuRead, DekuWrite, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deku(ctx = "api_index: u8", id = "api_index")]
pub enum GrappleDeviceInfo {
  #[deku(id = "0")]
  EnumerateRequest,

  #[deku(id = "1")]
  EnumerateResponse {
    model_id: GrappleModelId,
    firmware_version: [u8; 3],
    serial: u32,

    #[deku(assert = "*name_len <= 16")]
    name_len: u8,
    #[deku(count = "name_len")]
    name: Vec<u8>
  },

  #[deku(id = "2")]
  Blink {
    serial: u32
  },

  #[deku(id = "3")]
  SetName {
    serial: u32,
    #[deku(assert = "*name_len <= 16")]
    name_len: u8,
    #[deku(count = "name_len")]
    name: Vec<u8>
  }
}

#[cfg(test)]
mod test {
  extern crate alloc; 

  use alloc::vec;
  use deku::{DekuRead, bitvec::BitSlice, bitvec::bitvec, bitvec::Msb0, DekuWrite, DekuEnumExt};

  use super::GrappleDeviceInfo;

  #[test]
  fn test_enumerate_request() {
    let data = vec![];
    let bvec = BitSlice::from_slice(&data);
    let result = GrappleDeviceInfo::read(bvec, 0x00).unwrap();
    assert_eq!(result.1, GrappleDeviceInfo::EnumerateRequest);
  }

  #[test]
  fn test_enumerate_response() {
    let from = GrappleDeviceInfo::EnumerateResponse { model_id: super::GrappleModelId::SpiderLan, firmware_version: [0xA, 0xB, 0xC], serial: 0xDEADBEEF, name_len: 5, name: "Hello".as_bytes().to_vec() };
    let mut output = bitvec![u8, Msb0;];
    let ctx = 0;
    from.write(&mut output, ctx).unwrap();
    
    let decoded: (&BitSlice<u8, Msb0>, GrappleDeviceInfo) = GrappleDeviceInfo::read(&output, from.deku_id().unwrap()).unwrap();
    assert_eq!(decoded.1, from);
  }
}