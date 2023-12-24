use binmarshal::BinMarshal;

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
#[repr(C)]
pub enum GrappleError<T: BinMarshal<()>> {
  #[marshal(tag = "0")]
  ParameterOutOfBounds(alloc::string::String),
  #[marshal(tag = "0xFE")]
  Generic(String),
  #[marshal(tag = "0xFF")]
  Other(T)
}

#[cfg(feature = "std")]
impl<T: BinMarshal<()> + std::fmt::Display> std::fmt::Display for GrappleError<T> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      GrappleError::ParameterOutOfBounds(oob) => write!(f, "Parameter Out of Bounds: {}", oob),
      GrappleError::Generic(str) => write!(f, "Generic Error: {}", str),
      GrappleError::Other(other) => write!(f, "Other Error: {}", other),
    }
  }
}

#[cfg(feature = "std")]
impl<T: BinMarshal<()>, E: std::error::Error> From<E> for GrappleError<T> {
  fn from(value: E) -> Self {
    Self::Generic(format!("{}", value))
  }
}

#[cfg(feature = "std")]
impl<T: BinMarshal<()> + std::fmt::Display> From<GrappleError<T>> for anyhow::Error {
  fn from(value: GrappleError<T>) -> Self {
    anyhow::Error::msg(alloc::format!("{}", value))
  }
}

pub type GrappleResult<T, O = ()> = core::result::Result<T, GrappleError<O>>;
