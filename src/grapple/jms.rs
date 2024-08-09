use binmarshal::{AsymmetricCow, BitSpecification, Demarshal, LengthTaggedSlice, Marshal, MarshalUpdate, Payload, Proxy};
use bounded_static::ToStatic;

use super::GrappleMessageId;

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
pub enum JMSRole {
  #[marshal(tag = "0")]
  ScoringTable,
  #[marshal(tag = "1")]
  Red(u8),
  #[marshal(tag = "2")]
  Blue(u8),
  #[marshal(tag = "3")]
  TimerRed,
  #[marshal(tag = "4")]
  TimerBlue
}

/* STATUS */

#[derive(Debug, Copy, Clone, PartialEq, Eq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
pub enum JMSCardStatus {
  #[marshal(tag = "0")]
  IO(
    #[marshal(bits = 1)]
    [bool; 8]
  ),
  #[marshal(tag = "1")]
  Lighting,
}


#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(C)]
pub struct JMSElectronicsStatus {
  pub role: JMSRole,
  pub cards: [JMSCardStatus; 2]
}

/* UPDATE */

#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, Eq, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(C)]
pub struct Colour {
  pub red: u8,
  pub green: u8,
  pub blue: u8
}

impl Colour {
  pub fn new(red: u8, green: u8, blue: u8) -> Colour {
    Colour { red, green, blue }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
pub enum Pattern {
  #[marshal(tag = "0")]
  Blank,
  #[marshal(tag = "1")]
  Solid(Colour),
  #[marshal(tag = "2")]
  DiagonalStripes(Colour, Colour),
  #[marshal(tag = "3")]
  FillLeft(Colour, Colour, u8),
  #[marshal(tag = "4")]
  FillRight(Colour, Colour, u8)
}

#[derive(Debug, Clone, PartialEq, Eq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(tag_type = "u8")]
pub enum JMSCardUpdate<'a> {
  #[marshal(tag = "0")]
  IO(
    /* TODO */
  ),
  #[marshal(tag = "1")]
  Lighting {
    text_back: AsymmetricCow<'a, str>,
    text_back_colour: Colour,
    back_background: Pattern,
    text: AsymmetricCow<'a, str>,
    text_colour: Colour,
    bottom_bar: Pattern,
    top_bar: Pattern,
    background: Pattern,
  },
}

#[derive(Debug, Clone, PartialEq, Marshal, Demarshal, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(C)]
pub struct JMSElectronicsUpdate<'a> {
  pub card: u8,
  pub update: JMSCardUpdate<'a>
}

/* ROOT MESSAGE */

#[derive(Clone, Debug, PartialEq, Marshal, Demarshal, MarshalUpdate, ToStatic)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type", content = "data"))] 
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[marshal(ctx = GrappleMessageId, tag = "ctx.api_index")]
#[repr(C)]
pub enum JMSMessage<'a> {
  #[marshal(tag = "0")]
  Status(JMSElectronicsStatus),

  #[marshal(tag = "1")]
  SetRole(JMSRole),

  #[marshal(tag = "2")]
  Update(JMSElectronicsUpdate<'a>),

  #[marshal(tag = "3")]
  Blink
}