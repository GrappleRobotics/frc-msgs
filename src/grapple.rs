use crate::{FrcCanDecodable, DEVICE_BROADCAST, DEVICE_ID_BROADCAST, FrcCanEncodable, DEVICE_FIRMWARE_UPDATE, DEVICE_ULTRASONIC, DEVICE_IO_BREAKOUT};

pub const GRAPPLE_MANUFACTURER: u8 = 0x06;
pub const DEVICE_INFO_CLASS: u8 = 0x00;
pub const FIRMWARE_UPDATE_CLASS: u8 = 0x01;

fn read_u32(data: &[u8]) -> u32 {
  u32::from_le_bytes([ data[0], data[1], data[2], data[3] ])
}

fn read_u16(data: &[u8]) -> u16 {
  u16::from_le_bytes([ data[0], data[1] ])
}

/* DEVICE INFO */

#[derive(Debug, Clone, Copy, PartialEq, strum_macros::FromRepr)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum GrappleModelId {
  LaserCan = 0,
  SpiderCan = 1
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrappleDeviceInfo {
  EnumerateRequest,
  EnumerateResponse { device_id: u8, model_id: GrappleModelId, firmware_version: [u8; 3], serial: u32 },
  SetId { device_id: u8, serial: u32 },
  Blink { serial: u32 }
}

impl FrcCanDecodable for GrappleDeviceInfo {
  fn decode(data: &crate::FrcCanData) -> Option<Self> {
    if data.id.manufacturer != GRAPPLE_MANUFACTURER || data.id.device_type != DEVICE_BROADCAST || data.id.api_class != DEVICE_INFO_CLASS { return None; }
    match (data.id.api_index, data.id.device_id) {
      (0x00, DEVICE_ID_BROADCAST) => Some(GrappleDeviceInfo::EnumerateRequest),
      (0x00, device_id) if data.len == 8 => Some(GrappleDeviceInfo::EnumerateResponse { device_id, model_id: GrappleModelId::from_repr(data.data[0])?, firmware_version: [data.data[1], data.data[2], data.data[3]], serial: read_u32(&data.data[4..]) }),
      (0x01, device_id) if data.len == 4 => Some(GrappleDeviceInfo::SetId { device_id, serial: read_u32(&data.data) }),
      (0x02, DEVICE_ID_BROADCAST) if data.len == 4 => Some(GrappleDeviceInfo::Blink { serial: read_u32(&data.data) }),
      _ => None
    }
  }
}

impl FrcCanEncodable for GrappleDeviceInfo {
  fn encode(&self) -> crate::FrcCanData {
    let mut id = crate::FrcCanId {
      device_type: DEVICE_BROADCAST, manufacturer: GRAPPLE_MANUFACTURER,
      api_class: DEVICE_INFO_CLASS, api_index: 0x00, device_id: 0x00
    };
    let mut data = [0u8; 8];

    match self {
      GrappleDeviceInfo::EnumerateRequest => {
        id.api_index = 0x00;
        id.device_id = DEVICE_ID_BROADCAST;
        crate::FrcCanData { id, data, len: 0 }
      },
      GrappleDeviceInfo::EnumerateResponse { device_id, model_id, firmware_version, serial } => {
        id.api_index = 0x00;
        id.device_id = *device_id;
        data[0] = *model_id as u8;
        data[1..=3].copy_from_slice(firmware_version.as_slice());
        data[4..].copy_from_slice(serial.to_le_bytes().as_slice());
        crate::FrcCanData { id, data, len: 8 }
      },
      GrappleDeviceInfo::SetId { device_id, serial } => {
        id.api_index = 0x01;
        id.device_id = *device_id;
        data[0..=3].copy_from_slice(serial.to_le_bytes().as_slice());
        crate::FrcCanData { id, data, len: 4 }
      },
      GrappleDeviceInfo::Blink { serial } => {
        id.api_index = 0x02;
        id.device_id = DEVICE_ID_BROADCAST;
        data[0..=3].copy_from_slice(serial.to_le_bytes().as_slice());
        crate::FrcCanData { id, data, len: 4 }
      }
    }
  }
}

/* FIRMWARE */

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrappleFirmware {
  UpdateRequest { serial: u32 },
  UpdateReady { serial: u32, model_id: GrappleModelId },
  UpdatePart { serial: u32, data: [u8; 4], len: u8 },
  UpdatePartAck { serial: u32 },
  UpdateDone { serial: u32 }
}

impl FrcCanDecodable for GrappleFirmware {
  fn decode(data: &crate::FrcCanData) -> Option<Self> {
    if data.id.manufacturer != GRAPPLE_MANUFACTURER || data.id.device_type != DEVICE_FIRMWARE_UPDATE || data.id.api_class != FIRMWARE_UPDATE_CLASS || data.id.device_id != DEVICE_ID_BROADCAST { return None; }
    match data.id.api_index {
      0x00 if data.len == 4 => Some(Self::UpdateRequest { serial: read_u32(&data.data) }),
      0x01 if data.len == 4 => Some(Self::UpdateReady { serial: read_u32(&data.data), model_id: GrappleModelId::from_repr(data.data[4]).unwrap() }),
      0x02 => Some(Self::UpdatePart { serial: read_u32(&data.data), data: [data.data[4], data.data[5], data.data[6], data.data[7]], len: data.len - 4 }),
      0x03 if data.len == 4 => Some(Self::UpdatePartAck { serial: read_u32(&data.data) }),
      0x04 if data.len == 4 => Some(Self::UpdateDone { serial: read_u32(&data.data) }),
      _ => None
    }
  }
}

impl FrcCanEncodable for GrappleFirmware {
  fn encode(&self) -> crate::FrcCanData {
    let mut id = crate::FrcCanId {
      device_type: DEVICE_FIRMWARE_UPDATE, manufacturer: GRAPPLE_MANUFACTURER,
      api_class: FIRMWARE_UPDATE_CLASS, api_index: 0x00, device_id: DEVICE_ID_BROADCAST
    };
    let mut data = [0u8; 8];

    match self {
      GrappleFirmware::UpdateRequest { serial } => {
        id.api_index = 0x00;
        data[0..=3].copy_from_slice(serial.to_le_bytes().as_slice());
        crate::FrcCanData { id, data, len: 4 }
      },
      GrappleFirmware::UpdateReady { serial, model_id } => {
        id.api_index = 0x01;
        data[0..=3].copy_from_slice(serial.to_le_bytes().as_slice());
        data[4] = *model_id as u8;
        crate::FrcCanData { id, data, len: 4 }
      },
      GrappleFirmware::UpdatePart { serial, data: fw_data, len } => {
        id.api_index = 0x02;
        data[0..=3].copy_from_slice(serial.to_le_bytes().as_slice());
        data[4..=7].copy_from_slice(fw_data.as_slice());
        crate::FrcCanData { id, data: data.clone(), len: 4 + *len }
      },
      GrappleFirmware::UpdatePartAck { serial } => {
        id.api_index = 0x03;
        data[0..=3].copy_from_slice(serial.to_le_bytes().as_slice());
        crate::FrcCanData { id, data, len: 4 }
      },
      GrappleFirmware::UpdateDone { serial } => {
        id.api_index = 0x04;
        data[0..=3].copy_from_slice(serial.to_le_bytes().as_slice());
        crate::FrcCanData { id, data, len: 4 }
      },
    }
  }
}

/* LASERCAN */
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrappleLaserCanRoi {
  pub x: u8,
  pub y: u8,
  pub w: u8,
  pub h: u8
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrappleLaserCan {
  Status { device_id: u8, status: u8, distance_mm: u16, ambient: u16, long: bool, budget_ms: u8, roi: GrappleLaserCanRoi },
  SetRange { device_id: u8, long: bool },
  SetRoi { device_id: u8, roi: GrappleLaserCanRoi },
  SetTimingBudget { device_id: u8, budget_ms: u8 },
  SaveConfig { device_id: u8 }
}

impl FrcCanDecodable for GrappleLaserCan {
  fn decode(data: &crate::FrcCanData) -> Option<Self> {
    if data.id.manufacturer != GRAPPLE_MANUFACTURER || data.id.device_type != DEVICE_ULTRASONIC { return None; }
    match (data.id.api_class, data.id.api_index) {
      (0x20, 0x00) if data.len == 8 => Some(GrappleLaserCan::Status {
        device_id: data.id.device_id,
        status: data.data[0],
        distance_mm: read_u16(&data.data[1..]),
        ambient: read_u16(&data.data[3..]),
        long: (data.data[5] & 0b1000_0000) != 0,
        budget_ms: data.data[5] & 0b0111_1111,
        roi: GrappleLaserCanRoi {
          x: (data.data[6] >> 4) & 0b1111,
          y: (data.data[6]) & 0b1111,
          w: ((data.data[7] >> 4) & 0b1111) + 4,
          h: ((data.data[7]) & 0b1111) + 4
        }
      }),
      (0x21, 0x00) if data.len == 1 => Some(GrappleLaserCan::SetRange { device_id: data.id.device_id, long: data.data[0] != 0 }),
      (0x21, 0x01) if data.len == 4 => Some(GrappleLaserCan::SetRoi { device_id: data.id.device_id, roi: GrappleLaserCanRoi { x: data.data[0], y: data.data[1], w: data.data[2], h: data.data[3] } }),
      (0x21, 0x02) if data.len == 1 => Some(GrappleLaserCan::SetTimingBudget { device_id: data.id.device_id, budget_ms: data.data[0] }),
      (0x30, 0x00) => Some(GrappleLaserCan::SaveConfig { device_id: data.id.device_id }),
      _ => None
    }
  }
}

impl FrcCanEncodable for GrappleLaserCan {
  fn encode(&self) -> crate::FrcCanData {
    let mut id = crate::FrcCanId {
      device_type: DEVICE_ULTRASONIC, manufacturer: GRAPPLE_MANUFACTURER,
      api_class: 0x00, api_index: 0x00, device_id: 0x00
    };
    let mut data = [0u8; 8];

    match self {
      GrappleLaserCan::Status { device_id, status, distance_mm, ambient, long, budget_ms, roi } => {
        id.device_id = *device_id;
        id.api_class = 0x20;
        id.api_index = 0x00;
        data[0] = *status;
        data[1..=2].copy_from_slice(distance_mm.to_le_bytes().as_slice());
        data[3..=4].copy_from_slice(ambient.to_le_bytes().as_slice());
        data[5] = (*long as u8) << 7 | (*budget_ms & 0b0111_1111);
        data[6] = ((roi.x & 0b1111) << 4) | (roi.y & 0b1111);
        data[7] = (((roi.w - 4) & 0b1111) << 4) | ((roi.h - 4) & 0b1111);
        crate::FrcCanData { id, data, len: 8 }
      },
      GrappleLaserCan::SetRange { device_id, long } => {
        id.device_id = *device_id;
        id.api_class = 0x21;
        id.api_index = 0x00;
        data[0] = if *long { 1 } else { 0 };
        crate::FrcCanData { id, data, len: 1 }
      },
      GrappleLaserCan::SetRoi { device_id, roi } => {
        id.device_id = *device_id;
        id.api_class = 0x21;
        id.api_index = 0x01;
        data[0] = roi.x;
        data[1] = roi.y;
        data[2] = roi.w;
        data[3] = roi.h;
        crate::FrcCanData { id, data, len: 4 }
      },
      GrappleLaserCan::SetTimingBudget { device_id, budget_ms } => {
        id.device_id = *device_id;
        id.api_class = 0x21;
        id.api_index = 0x02;
        data[0] = *budget_ms;
        crate::FrcCanData { id, data, len: 1 }
      },
      GrappleLaserCan::SaveConfig { device_id } => {
        id.device_id = *device_id;
        id.api_class = 0x30;
        id.api_index = 0x00;
        crate::FrcCanData { id, data, len: 0 }
      }
    }
  }
}

/* SPIDERCAN */
#[derive(Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpiderCanPull {
  NoPull,
  PullUp,
  PullDown,
}

#[derive(Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpiderCanOutputMode {
  PushPull,
  OpenDrain
}

#[derive(Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpiderCanPinMode {
  DigitalIn(SpiderCanPull),
  DigitalOut(SpiderCanOutputMode),
  Analog
}

impl From<u8> for SpiderCanPinMode {
  fn from(value: u8) -> Self {
    match value {
      0b000 => SpiderCanPinMode::DigitalIn(SpiderCanPull::NoPull),
      0b001 => SpiderCanPinMode::DigitalIn(SpiderCanPull::PullUp),
      0b010 => SpiderCanPinMode::DigitalIn(SpiderCanPull::PullDown),
      0b100 => SpiderCanPinMode::DigitalOut(SpiderCanOutputMode::PushPull),
      0b101 => SpiderCanPinMode::DigitalOut(SpiderCanOutputMode::OpenDrain),
      0b011 => SpiderCanPinMode::Analog,
      _ => SpiderCanPinMode::DigitalIn(SpiderCanPull::PullUp)
    }
  }
}

impl Into<u8> for SpiderCanPinMode {
  fn into(self) -> u8 {
    match self {
      SpiderCanPinMode::DigitalIn(SpiderCanPull::NoPull) => 0b000,
      SpiderCanPinMode::DigitalIn(SpiderCanPull::PullUp) => 0b001,
      SpiderCanPinMode::DigitalIn(SpiderCanPull::PullDown) => 0b010,
      SpiderCanPinMode::DigitalOut(SpiderCanOutputMode::PushPull) => 0b100,
      SpiderCanPinMode::DigitalOut(SpiderCanOutputMode::OpenDrain) => 0b101,
      SpiderCanPinMode::Analog => 0b011,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GrappleSpiderCan {
  StatusDigital { device_id: u8, pin_status: [bool; 8], pin_modes: [SpiderCanPinMode; 8] },
  StatusAnalog { device_id: u8, frame_number: u8, pin_status: [u16; 4] },
  ConfigurePin { device_id: u8, pin_id: u8, mode: SpiderCanPinMode },
  SaveConfig { device_id: u8 }
}

impl FrcCanDecodable for GrappleSpiderCan {
  fn decode(data: &crate::FrcCanData) -> Option<Self> {
    if data.id.manufacturer != GRAPPLE_MANUFACTURER || data.id.device_type != DEVICE_IO_BREAKOUT { return None; }
    match (data.id.api_class, data.id.api_index) {
      (0x20, 0x00) if data.len == 5 => Some(GrappleSpiderCan::StatusDigital {
        device_id: data.id.device_id,
        pin_status: [data.data[0] & 0b1 != 0, data.data[0] & 0b10 != 0, data.data[0] & 0b100 != 0, data.data[0] & 0b1000 != 0,
                     data.data[0] & 0b1_0000 != 0, data.data[0] & 0b10_0000 != 0, data.data[0] & 0b100_0000 != 0, data.data[0] & 0b1000_0000 != 0],
        pin_modes:  [(data.data[1] & 0b1111).into(), ((data.data[1] >> 4) & 0b1111).into(),
                     (data.data[2] & 0b1111).into(), ((data.data[2] >> 4) & 0b1111).into(),
                     (data.data[3] & 0b1111).into(), ((data.data[3] >> 4) & 0b1111).into(),
                     (data.data[4] & 0b1111).into(), ((data.data[4] >> 4) & 0b1111).into()]
      }),
      (0x21, frame_num) if data.len == 8 => Some(GrappleSpiderCan::StatusAnalog {
        device_id: data.id.device_id,
        frame_number: frame_num,
        pin_status: [ read_u16(&data.data[0..=1]), read_u16(&data.data[2..=3]),
                      read_u16(&data.data[4..=5]), read_u16(&data.data[6..=7]) ]
      }),
      (0x22, 0x00) if data.len == 2 => Some(GrappleSpiderCan::ConfigurePin {
        device_id: data.id.device_id,
        pin_id: data.data[0],
        mode: data.data[1].into()
      }),
      (0x30, 0x00) => Some(GrappleSpiderCan::SaveConfig { device_id: data.id.device_id }),
      _ => None
    }
  }
}

impl FrcCanEncodable for GrappleSpiderCan {
  fn encode(&self) -> crate::FrcCanData {
    let mut id = crate::FrcCanId {
      device_type: DEVICE_IO_BREAKOUT, manufacturer: GRAPPLE_MANUFACTURER,
      api_class: 0x00, api_index: 0x00, device_id: 0x00
    };
    let mut data = [0u8; 8];

    match self {
      GrappleSpiderCan::StatusDigital { device_id, pin_status, pin_modes } => {
        id.device_id = *device_id;
        id.api_class = 0x20;
        id.api_index = 0x00;
        for i in 0..pin_status.len() {
          if pin_status[i] {
            data[0] |= 1 << i;
          }
        }
        data[1] = Into::<u8>::into(pin_modes[0]) | (Into::<u8>::into(pin_modes[1]) << 4);
        data[2] = Into::<u8>::into(pin_modes[2]) | (Into::<u8>::into(pin_modes[3]) << 4);
        data[3] = Into::<u8>::into(pin_modes[4]) | (Into::<u8>::into(pin_modes[5]) << 4);
        data[4] = Into::<u8>::into(pin_modes[6]) | (Into::<u8>::into(pin_modes[7]) << 4);
        crate::FrcCanData { id, data, len: 5 }
      },
      GrappleSpiderCan::StatusAnalog { device_id, frame_number, pin_status } => {
        id.device_id = *device_id;
        id.api_class = 0x21;
        id.api_index = *frame_number;
        data[0..=1].clone_from_slice(&pin_status[0].to_le_bytes());
        data[2..=3].clone_from_slice(&pin_status[1].to_le_bytes());
        data[4..=5].clone_from_slice(&pin_status[2].to_le_bytes());
        data[6..=7].clone_from_slice(&pin_status[3].to_le_bytes());
        crate::FrcCanData { id, data, len: 8 }
      },
      GrappleSpiderCan::ConfigurePin { device_id, pin_id, mode } => {
        id.device_id = *device_id;
        id.api_class = 0x22;
        id.api_index = 0x00;
        data[0] = *pin_id;
        data[1] = (*mode).into();
        crate::FrcCanData { id, data, len: 2 }
      },
      GrappleSpiderCan::SaveConfig { device_id } => {
        id.device_id = *device_id;
        id.api_class = 0x30;
        id.api_index = 0x00;
        crate::FrcCanData { id, data, len: 0 }
      }
    }
  }
}

/* GRAPPLE */

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Grapple {
  DeviceInfo(GrappleDeviceInfo),
  Firmware(GrappleFirmware),
  LaserCan(GrappleLaserCan),
  SpiderCan(GrappleSpiderCan),
}

impl FrcCanDecodable for Grapple {
  fn decode(data: &crate::FrcCanData) -> Option<Self> {
    GrappleDeviceInfo::decode(data).map(|x| Grapple::DeviceInfo(x))
    .or(GrappleFirmware::decode(data).map(|x| Grapple::Firmware(x)))
    .or(GrappleLaserCan::decode(data).map(|x| Grapple::LaserCan(x)))
    .or(GrappleSpiderCan::decode(data).map(|x| Grapple::SpiderCan(x)))
  }
}

impl FrcCanEncodable for Grapple {
  fn encode(&self) -> crate::FrcCanData {
    match self {
      Grapple::DeviceInfo(di) => di.encode(),
      Grapple::Firmware(fw) => fw.encode(),
      Grapple::LaserCan(lc) => lc.encode(),
      Grapple::SpiderCan(sc) => sc.encode(),
    }
  }
}

/* ENCODE DECODE TESTS */

#[cfg(test)]
mod test {
  use crate::{FrcCanDecodable, FrcCanEncodable};
  use super::{Grapple, SpiderCanPinMode, SpiderCanPull, SpiderCanOutputMode};

  fn assert_encode_decode(data: Grapple) {
    assert_eq!(Some(data.clone()), Grapple::decode(&data.encode()));
  }

  #[test]
  fn test_device_info() {
    assert_encode_decode(Grapple::DeviceInfo(super::GrappleDeviceInfo::EnumerateRequest));
    assert_encode_decode(Grapple::DeviceInfo(super::GrappleDeviceInfo::EnumerateResponse { device_id: 0x02, model_id: super::GrappleModelId::LaserCan, firmware_version: [1, 2, 0], serial: 0xDEADBEEF }));
    assert_encode_decode(Grapple::DeviceInfo(super::GrappleDeviceInfo::SetId { device_id: 0x02, serial: 0xDEADBEEF }));
    assert_encode_decode(Grapple::DeviceInfo(super::GrappleDeviceInfo::Blink { serial: 0xDEADBEEF }));
  }

  #[test]
  fn test_firmware() {
    assert_encode_decode(Grapple::Firmware(super::GrappleFirmware::UpdateRequest { serial: 0xDEADBEEF }));
    assert_encode_decode(Grapple::Firmware(super::GrappleFirmware::UpdateReady { serial: 0xDEADBEEF, model_id: super::GrappleModelId::SpiderCan }));
    assert_encode_decode(Grapple::Firmware(super::GrappleFirmware::UpdatePart { serial: 0xDEADBEEF, data: [0, 1, 2, 0], len: 3 }));
    assert_encode_decode(Grapple::Firmware(super::GrappleFirmware::UpdatePartAck { serial: 0xDEADBEEF }));
    assert_encode_decode(Grapple::Firmware(super::GrappleFirmware::UpdateDone { serial: 0xDEADBEEF }));
  }

  #[test]
  fn test_lasercan() {
    assert_encode_decode(Grapple::LaserCan(super::GrappleLaserCan::Status { device_id: 0x01, status: 0x02, distance_mm: 0x1234, ambient: 0x4567, long: true, budget_ms: 34, roi: super::GrappleLaserCanRoi { x: 8, y: 7, w: 16, h: 4 } }));
    assert_encode_decode(Grapple::LaserCan(super::GrappleLaserCan::SetRange { device_id: 0x02, long: true }));
    assert_encode_decode(Grapple::LaserCan(super::GrappleLaserCan::SetRoi { device_id: 0x02, roi: super::GrappleLaserCanRoi { x: 1, y: 2, w: 3, h: 4 } }));
    assert_encode_decode(Grapple::LaserCan(super::GrappleLaserCan::SetTimingBudget { device_id: 0x02, budget_ms: 100 }));
  }

  #[test]
  fn test_spidercan() {
    assert_encode_decode(Grapple::SpiderCan(super::GrappleSpiderCan::ConfigurePin { device_id: 0x01, pin_id: 0x10, mode: super::SpiderCanPinMode::DigitalOut(super::SpiderCanOutputMode::OpenDrain) }));
    assert_encode_decode(Grapple::SpiderCan(super::GrappleSpiderCan::StatusAnalog { device_id: 0x02, frame_number: 0x01, pin_status: [100, 200, 300, 400] }));
    assert_encode_decode(Grapple::SpiderCan(
      super::GrappleSpiderCan::StatusDigital {
        device_id: 0x03,
        pin_status: [true, true, false, false, true, false, false, true],
        pin_modes: [ SpiderCanPinMode::Analog, SpiderCanPinMode::DigitalIn(SpiderCanPull::NoPull), SpiderCanPinMode::DigitalIn(SpiderCanPull::PullDown), SpiderCanPinMode::DigitalIn(SpiderCanPull::PullUp), 
                     SpiderCanPinMode::DigitalOut(SpiderCanOutputMode::OpenDrain), SpiderCanPinMode::DigitalOut(SpiderCanOutputMode::PushPull), SpiderCanPinMode::Analog, SpiderCanPinMode::Analog ] 
      }
    ));
  }
}