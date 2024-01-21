use alloc::borrow::Cow;
use binmarshal::{Marshal, Demarshal, MarshalUpdate, AsymmetricCow};
use bounded_static::ToStatic;

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
#[repr(C)]
pub enum GrappleError<'a> {
  #[marshal(tag = "0x00")]
  ParameterOutOfBounds(
    #[cfg_attr(feature = "serde", serde(borrow))]
    AsymmetricCow<'a, str>
  ),
  
  #[marshal(tag = "0x01")]
  FailedAssertion(
    #[cfg_attr(feature = "serde", serde(borrow))]
    AsymmetricCow<'a, str>
  ),

  #[marshal(tag = "0xFE")]
  TimedOut(
    #[cfg_attr(feature = "serde", serde(borrow))]
    AsymmetricCow<'a, str>
  ),

  #[marshal(tag = "0xFF")]
  Generic(
    #[cfg_attr(feature = "serde", serde(borrow))]
    AsymmetricCow<'a, str>
  ),
}

#[cfg(feature = "std")]
impl<'a> std::fmt::Display for GrappleError<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      GrappleError::ParameterOutOfBounds(oob) => write!(f, "Parameter Out of Bounds: {}", oob.as_ref()),
      GrappleError::FailedAssertion(msg) => write!(f, "Failed Assertion: {}", msg.as_ref()),
      GrappleError::TimedOut(msg) => write!(f, "Timed Out: {}", msg.as_ref()),
      GrappleError::Generic(str) => write!(f, "Generic Error: {}", str.as_ref()),
    }
  }
}

// TODO: Build in get_tag() into binmarshal for this.
impl<'a> GrappleError<'a> {
  pub fn to_error_code(&self) -> u8 {
    match self {
      GrappleError::ParameterOutOfBounds(_) => 0x00,
      GrappleError::FailedAssertion(_) => 0x01,
      GrappleError::TimedOut(_) => 0xFE,
      GrappleError::Generic(_) => 0xFF,
    }
  }
}

#[cfg(feature = "std")]
impl<'a, E: std::error::Error> From<E> for GrappleError<'a> {
  fn from(value: E) -> Self {
    Self::Generic(AsymmetricCow(Cow::Owned(format!("{}", value))))
  }
}

#[cfg(feature = "std")]
impl<'a> From<GrappleError<'a>> for anyhow::Error {
  fn from(value: GrappleError) -> Self {
    anyhow::Error::msg(alloc::format!("{}", value))
  }
}

pub type GrappleResult<'a, T> = core::result::Result<T, GrappleError<'a>>;
