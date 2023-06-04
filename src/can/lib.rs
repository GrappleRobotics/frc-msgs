#![cfg_attr(not(feature="std"), no_std)]

pub mod ni;
pub mod grapple;

use bxcan::{ExtendedId, Frame, Id, Data};
use grapple::Grapple;
use ni::Ni;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrcCanId {
  pub device_type: u8,
  pub manufacturer: u8,
  pub api_class: u8,
  pub api_index: u8,
  pub device_id: u8
}

impl FrcCanId {
  pub fn new(device_type: u8, manufacturer: u8, api_class: u8, api_index: u8, device_id: u8) -> Self {
    Self {
      device_type, manufacturer, api_class, api_index, device_id
    }
  }
}

impl Into<u32> for FrcCanId {
  fn into(self) -> u32 {
    ((self.device_type as u8 as u32 & 0b11111) << 24) 
      | ((self.manufacturer as u8 as u32 & 0b11111111) << 16)
      | ((self.api_class as u32 & 0b111111) << 10)
      | ((self.api_index as u32 & 0b1111) << 6)
      | (self.device_id as u32 & 0b111111)
  }
}

impl From<u32> for FrcCanId {
  fn from(raw: u32) -> Self {
    Self {
      device_type: ((raw >> 24) & 0b11111) as u8,
      manufacturer: ((raw >> 16) & 0b11111111) as u8,
      api_class: ((raw >> 10) & 0b111111) as u8,
      api_index: ((raw >> 6) & 0b1111) as u8,
      device_id: (raw & 0b111111) as u8,
    }
  }
}

impl Into<ExtendedId> for FrcCanId {
  fn into(self) -> ExtendedId {
    let id: u32 = self.into();
    ExtendedId::new(id).unwrap()
  }
}

impl From<ExtendedId> for FrcCanId {
  fn from(value: ExtendedId) -> Self {
    let raw = value.as_raw();
    Self::from(raw)
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrcCanData {
  pub id: FrcCanId,
  pub data: [u8; 8],
  pub len: u8
}

impl FrcCanData {
  pub fn from_frame(frame: &Frame) -> Option<FrcCanData> {
    match (frame.id(), frame.data()) {
      (Id::Extended(id), Some(data)) => {
        let mut fcd = FrcCanData { id: id.into(), data: [0u8; 8], len: data.len() as u8 };
        for (i, d) in data.iter().enumerate() {
          fcd.data[i] = *d;
        }
        Some(fcd)
      },
      _ => None
    }
  }

  pub fn to_frame(&self) -> Frame {
    Frame::new_data(Into::<ExtendedId>::into(self.id.clone()), Data::new(&self.data[0..self.len as usize]).unwrap())
  }
}

pub trait FrcCanEncodable {
  fn encode(&self) -> FrcCanData;
}

pub trait FrcCanDecodable : Sized {
  fn decode(data: &FrcCanData) -> Option<Self>;
}

pub const DEVICE_BROADCAST: u8 = 0;
pub const DEVICE_FIRMWARE_UPDATE: u8 = 31;

pub const DEVICE_ROBOT_CONTROLLER: u8 = 1;
pub const DEVICE_MOTOR_CONTROLLER: u8 = 2;
pub const DEVICE_RELAY_CONTROLLER: u8 = 3;
pub const DEVICE_GYRO_SENSOR: u8 = 4;
pub const DEVICE_ACCELEROMETER: u8 = 5;
pub const DEVICE_ULTRASONIC: u8 = 6;
pub const DEVICE_GEARTOOTH: u8 = 7;
pub const DEVICE_PDP: u8 = 8;
pub const DEVICE_PNEUMATICS: u8 = 9;
pub const DEVICE_MISC: u8 = 10;
pub const DEVICE_IO_BREAKOUT: u8 = 11;

pub const DEVICE_ID_BROADCAST: u8 = 0x3F;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CanMessage {
  Ni(Ni),
  Grapple(Grapple)
}

impl FrcCanDecodable for CanMessage {
  fn decode(data: &FrcCanData) -> Option<Self> {
    Ni::decode(data).map(|x| CanMessage::Ni(x))
    .or(Grapple::decode(data).map(|x| CanMessage::Grapple(x)))
  }
}
