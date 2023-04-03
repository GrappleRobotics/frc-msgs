#![no_std]

use bxcan::ExtendedId;

pub const DEVICE_BROADCAST: u8 = 0;
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
pub const DEVICE_FIRMWARE_UPDATE: u8 = 31;

pub const MANUFACTURER_NI: u8 = 1;
pub const MANUFACTURER_GRAPPLE: u8 = 6;

/* DEVICE INFO */
/* Enumeration, Device Info, etc */
pub const API_DEVICE_INFO_CLASS: u8 = 0x00;
pub const API_DEVICE_INFO_ENUMERATE: u8  = 0x00;

pub const API_DEVICE_SET_ID_CLASS: u8 = 0x01;

/* BOOTLOADER */
pub const API_FIRMWARE_CLASS: u8 = 0x10;
pub const API_FIRMWARE_UPDATE_REQUEST: u8 = 0x00;
pub const API_FIRMWARE_UPDATE_READY: u8 = 0x01;
pub const API_FIRMWARE_UPDATE_PART: u8 = 0x02;
pub const API_FIRMWARE_UPDATE_DONE: u8 = 0x03;

pub const DEVICE_ID_BCAST: u8 = 0x3F;

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

  pub fn rio_heartbeat() -> Self {
    Self::new(DEVICE_ROBOT_CONTROLLER, MANUFACTURER_NI, 0x06, 0x02, 0x00)
  }

  pub fn is_rio_heartbeat(&self) -> bool {
    self.manufacturer == MANUFACTURER_NI && self.device_type == DEVICE_ROBOT_CONTROLLER && self.device_id == 0x00 && self.api_class == 0x06 && self.api_index == 0x02
  }
}

impl Into<ExtendedId> for FrcCanId {
  fn into(self) -> ExtendedId {
    let id = ((self.device_type as u8 as u32 & 0b11111) << 24) 
      | ((self.manufacturer as u8 as u32 & 0b11111111) << 16)
      | ((self.api_class as u32 & 0b111111) << 10)
      | ((self.api_index as u32 & 0b1111) << 6)
      | (self.device_id as u32 & 0b111111);
    
    ExtendedId::new(id).unwrap()
  }
}

impl From<ExtendedId> for FrcCanId {
  fn from(value: ExtendedId) -> Self {
    let raw = value.as_raw();
    Self {
      device_type: ((raw >> 24) & 0b11111) as u8,
      manufacturer: ((raw >> 16) & 0b11111111) as u8,
      api_class: ((raw >> 10) & 0b111111) as u8,
      api_index: ((raw >> 6) & 0b1111) as u8,
      device_id: (raw & 0b111111) as u8,
    }
  }
}

pub enum CanProcessOutcome {
  RioHeartbeat(),
  Reply(bxcan::Frame),
  SetId(u8),
  FieldUpgrade,
  Broadcast(FrcCanId, Option<bxcan::Data>),
  Targetted(FrcCanId, Option<bxcan::Data>),
  Other(FrcCanId, Option<bxcan::Data>),
  OtherManufacturer(FrcCanId, Option<bxcan::Data>)
}

#[derive(Clone, strum_macros::FromRepr)]
#[repr(u8)]
pub enum ModelId {
  LaserCAN = 0x0,
  SpiderCAN = 0x1,
}

pub struct CanSystem {
  device_type: u8,
  device_id: u8,
  model_id: ModelId,
  firmware_version: [u8; 3]
}

impl CanSystem {
  pub fn new(device_type: u8, device_id: u8, model_id: ModelId, firmware_version: [u8; 3]) -> Self {
    Self { device_type, device_id, model_id, firmware_version }
  }

  pub fn get_uid() -> &'static [u8; 12] {
    unsafe { &*(0x1FFF_F7AC as *const u8).cast::<[u8; 12]>() }
  }

  pub fn get_serial_hash() -> u32 {
    let mut hash = 0u32;
    for b in Self::get_uid() {
      hash = (hash.wrapping_mul(31)) ^ (*b as u32);
    }
    hash
  }

  pub fn process(&mut self, frame: &bxcan::Frame) -> Option<CanProcessOutcome> {
    if let bxcan::Id::Extended(eid) = frame.id() {
      let can_id: FrcCanId = eid.into();

      if can_id.is_rio_heartbeat() {
        Some(CanProcessOutcome::RioHeartbeat())
      } else if can_id.manufacturer == MANUFACTURER_GRAPPLE {
        let data = frame.data();
        match (can_id.device_type, can_id.api_class, can_id.api_index, can_id.device_id, data) {
          // Enumerate
          (DEVICE_BROADCAST, API_DEVICE_INFO_CLASS, API_DEVICE_INFO_ENUMERATE, DEVICE_ID_BCAST, _) => {
            let mut data = [0u8; 8];
            data[0] = self.model_id.clone() as u8;
            data[1..=3].copy_from_slice(&self.firmware_version);
            data[4..].copy_from_slice(&Self::get_serial_hash().to_le_bytes());
            Some(CanProcessOutcome::Reply(bxcan::Frame::new_data(
              Into::<bxcan::ExtendedId>::into(FrcCanId::new(DEVICE_BROADCAST, MANUFACTURER_GRAPPLE, API_DEVICE_INFO_CLASS, API_DEVICE_INFO_ENUMERATE, self.device_id)),
              data
            )))
          },
          // Set ID - this is a broadcast message, but the data itself links it to this device through the serial number
          (DEVICE_BROADCAST, API_DEVICE_SET_ID_CLASS, 0x00, DEVICE_ID_BCAST, Some(data)) if data.len() == 5 => {
            let target_serial = u32::from_le_bytes([ data[0], data[1], data[2], data[3] ]);
            if target_serial == Self::get_serial_hash() {
              self.device_id = data[4];
              Some(CanProcessOutcome::SetId(data[4]))
            } else {
              None
            }
          },
          // Field Upgrade - this is also a broadcast message, but the data links through serial number
          (DEVICE_FIRMWARE_UPDATE, API_FIRMWARE_CLASS, API_FIRMWARE_UPDATE_REQUEST, DEVICE_ID_BCAST, Some(data)) if data.len() == 4 => {
            let target_serial = u32::from_le_bytes([ data[0], data[1], data[2], data[3] ]);
            if target_serial == Self::get_serial_hash() {
              Some(CanProcessOutcome::FieldUpgrade)
            } else {
              None
            }
          },
          // Targetted device
          (device_type, api_class, api_id, device_id, data) if device_type == self.device_type && device_id == self.device_id => match (api_class, api_id, data) {
            (_, _, data) => Some(CanProcessOutcome::Targetted(can_id, data.cloned()))
          },
          // Generic broadcast
          (DEVICE_BROADCAST, _, _, DEVICE_ID_BCAST, data) => Some(CanProcessOutcome::Broadcast(can_id, data.cloned())),
          // Other message - not targetted to us, not a broadcast
          (_, _, _, _, data) => Some(CanProcessOutcome::Other(can_id, data.cloned()))
        }
      } else {
        Some(CanProcessOutcome::OtherManufacturer(can_id, frame.data().cloned()))
      }
    } else {
      None
    }
  }
}