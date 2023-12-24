use binmarshal::BinMarshal;

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
#[repr(C)]
pub enum GrappleError {
  #[marshal(tag = "0")]
  ParameterOutOfBounds(String),
  #[marshal(tag = "1")]
  FailedAssertion(String),
  #[marshal(tag = "0xFF")]
  Generic(String),
}

#[cfg(feature = "std")]
impl std::fmt::Display for GrappleError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      GrappleError::ParameterOutOfBounds(oob) => write!(f, "Parameter Out of Bounds: {}", oob),
      GrappleError::FailedAssertion(msg) => write!(f, "Failed Assertion: {}", msg),
      GrappleError::Generic(str) => write!(f, "Generic Error: {}", str),
    }
  }
}

#[cfg(feature = "std")]
impl<E: std::error::Error> From<E> for GrappleError {
  fn from(value: E) -> Self {
    Self::Generic(format!("{}", value))
  }
}

#[cfg(feature = "std")]
impl From<GrappleError> for anyhow::Error {
  fn from(value: GrappleError) -> Self {
    anyhow::Error::msg(alloc::format!("{}", value))
  }
}

pub type GrappleResult<T> = core::result::Result<T, GrappleError>;
