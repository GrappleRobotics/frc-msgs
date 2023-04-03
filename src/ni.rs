use crate::{FrcCanData, FrcCanDecodable};

#[derive(Debug, Clone)]
pub enum Ni {
  RioHeartbeat
}

impl FrcCanDecodable for Ni {
  fn decode(data: &FrcCanData) -> Option<Self> {
    if data.id.manufacturer != 0x01 { return None; }
    match (data.id.device_type, data.id.api_class, data.id.api_index) {
      (0x01, 0x06, 0x02) => Some(Ni::RioHeartbeat),
      _ => None
    }
  }
}
