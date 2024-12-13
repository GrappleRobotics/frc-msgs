use alloc::borrow::Cow;
use binmarshal::{Marshal, Demarshal, MarshalUpdate, AsymmetricCow};
use bounded_static::ToStatic;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "pyo3")]
use pyo3::BoundObject;

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


#[derive(Debug)]
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pyclass)]
#[pyo3(name = "GrappleResult")]
#[repr(C)]
pub struct GrappleResultPy {
  pub error: Option<String>,
  pub ok: Option<pyo3::PyObject>
}

#[cfg(feature = "pyo3")]
pub fn convert_grpl_result_to_py<'py, T: IntoPyObject<'py> + Clone>(py: Python<'py>, obj: GrappleResult<'_, T>) -> PyResult<GrappleResultPy>{
  match obj {
    Ok(v) => {
      return Ok(GrappleResultPy { error: None, ok: Some(v.into_pyobject(py).map_err(Into::into)?.into_any().unbind()) })
    },
    Err(e) => {
      return Ok(GrappleResultPy { error: Some(e.to_string()), ok: None })
    },
  }
}
