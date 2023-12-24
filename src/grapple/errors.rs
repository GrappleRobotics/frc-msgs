use core::ops::Deref;

use binmarshal::BinMarshal;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum CowStr {
  Borrowed(&'static str),
  Owned(String)
}

impl AsRef<str> for CowStr {
  fn as_ref(&self) -> &str {
    self.deref()
  }
}

impl Deref for CowStr {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    match self {
      CowStr::Borrowed(borrowed) => borrowed,
      CowStr::Owned(v) => v.as_str(),
    }
  }
}

impl PartialEq for CowStr {
  fn eq(&self, other: &Self) -> bool {
    self.deref() == other.deref()
  }
}

impl Eq for CowStr {}

#[cfg(feature = "serde")]
impl<'de, 'a> serde::Deserialize<'de> for CowStr { 
  #[inline] 
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
  where 
      D: serde::Deserializer<'de>, 
  { 
    String::deserialize(deserializer).map(CowStr::Owned) 
  } 
}

#[cfg(feature = "serde")]
impl serde::Serialize for CowStr {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer
  {
    match self {
      CowStr::Borrowed(borrowed) => borrowed.serialize(serializer),
      CowStr::Owned(owned) => owned.serialize(serializer),
    }
  }
}

impl BinMarshal<()> for CowStr {
  type Context = ();

  fn write<W: binmarshal::BitWriter>(&self, writer: &mut W, _ctx: ()) -> bool {
    writer.align(1);
    if let Some(arr) = writer.reserve_and_advance_aligned_slice(self.len() + 1) {
      let arr_str_bytes = &mut arr[0..self.len()];
      arr_str_bytes.copy_from_slice(&self.as_bytes()[..]);
      arr[arr.len() - 1] = 0;
      true
    } else {
      false
    }
  }

  fn read(view: &mut binmarshal::BitView<'_>, _ctx: ()) -> Option<Self> {
    String::read(view, _ctx).map(CowStr::Owned)
  }

  fn update(&mut self, _ctx: &mut ()) { }
}

#[derive(Debug, Clone, BinMarshal, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
#[repr(C)]
pub enum GrappleError {
  #[marshal(tag = "0")]
  ParameterOutOfBounds(CowStr),
  #[marshal(tag = "1")]
  FailedAssertion(CowStr),
  #[marshal(tag = "0xFF")]
  Generic(CowStr),
}

#[cfg(feature = "std")]
impl std::fmt::Display for GrappleError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      GrappleError::ParameterOutOfBounds(oob) => write!(f, "Parameter Out of Bounds: {}", oob.as_ref()),
      GrappleError::FailedAssertion(msg) => write!(f, "Failed Assertion: {}", msg.as_ref()),
      GrappleError::Generic(str) => write!(f, "Generic Error: {}", str.as_ref()),
    }
  }
}

#[cfg(feature = "std")]
impl<E: std::error::Error> From<E> for GrappleError {
  fn from(value: E) -> Self {
    Self::Generic(CowStr::Owned(format!("{}", value)))
  }
}

#[cfg(feature = "std")]
impl From<GrappleError> for anyhow::Error {
  fn from(value: GrappleError) -> Self {
    anyhow::Error::msg(alloc::format!("{}", value))
  }
}

pub type GrappleResult<T> = core::result::Result<T, GrappleError>;
