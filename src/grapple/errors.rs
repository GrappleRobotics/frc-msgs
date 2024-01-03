use binmarshal::{Marshal, Demarshal, MarshalUpdate, CowStr};

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, MarshalUpdate)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
#[repr(C)]
pub enum GrappleError<'a> {
  #[marshal(tag = "0")]
  ParameterOutOfBounds(
    #[cfg_attr(feature = "serde", serde(borrow))]
    CowStr<'a>
  ),
  
  #[marshal(tag = "1")]
  FailedAssertion(
    #[cfg_attr(feature = "serde", serde(borrow))]
    CowStr<'a>
  ),

  #[marshal(tag = "0xFF")]
  Generic(
    #[cfg_attr(feature = "serde", serde(borrow))]
    CowStr<'a>
  ),
}

#[cfg(feature = "std")]
impl<'a> std::fmt::Display for GrappleError<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      GrappleError::ParameterOutOfBounds(oob) => write!(f, "Parameter Out of Bounds: {}", oob.as_ref()),
      GrappleError::FailedAssertion(msg) => write!(f, "Failed Assertion: {}", msg.as_ref()),
      GrappleError::Generic(str) => write!(f, "Generic Error: {}", str.as_ref()),
    }
  }
}

#[cfg(feature = "std")]
impl<'a, E: std::error::Error> From<E> for GrappleError<'a> {
  fn from(value: E) -> Self {
    Self::Generic(CowStr::Owned(format!("{}", value)))
  }
}

#[cfg(feature = "std")]
impl<'a> From<GrappleError<'a>> for anyhow::Error {
  fn from(value: GrappleError) -> Self {
    anyhow::Error::msg(alloc::format!("{}", value))
  }
}

pub type GrappleResult<'a, T> = core::result::Result<T, GrappleError<'a>>;
