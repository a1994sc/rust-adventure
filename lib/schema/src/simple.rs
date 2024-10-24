#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
  #[doc = r" Error from a TryFrom or FromStr implementation."]
  pub struct ConversionError(std::borrow::Cow<'static, str>);
  impl std::error::Error for ConversionError {}
  impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
      std::fmt::Display::fmt(&self.0, f)
    }
  }
  impl std::fmt::Debug for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
      std::fmt::Debug::fmt(&self.0, f)
    }
  }
  impl From<&'static str> for ConversionError {
    fn from(value: &'static str) -> Self {
      Self(value.into())
    }
  }
  impl From<String> for ConversionError {
    fn from(value: String) -> Self {
      Self(value.into())
    }
  }
}
#[doc = "Simple"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Simple\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"completed\","]
#[doc = "    \"id\","]
#[doc = "    \"title\","]
#[doc = "    \"userId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"completed\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"userId\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Simple {
  pub completed: bool,
  pub id: f64,
  pub title: String,
  #[serde(rename = "userId")]
  pub user_id: f64,
}
impl From<&Simple> for Simple {
  fn from(value: &Simple) -> Self {
    value.clone()
  }
}
impl Simple {
  pub fn builder() -> builder::Simple {
    Default::default()
  }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
  #[derive(Clone, Debug)]
  pub struct Simple {
    completed: Result<bool, String>,
    id: Result<f64, String>,
    title: Result<String, String>,
    user_id: Result<f64, String>,
  }
  impl Default for Simple {
    fn default() -> Self {
      Self {
        completed: Err("no value supplied for completed".to_string()),
        id: Err("no value supplied for id".to_string()),
        title: Err("no value supplied for title".to_string()),
        user_id: Err("no value supplied for user_id".to_string()),
      }
    }
  }
  impl Simple {
    pub fn completed<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<bool>,
      T::Error: std::fmt::Display,
    {
      self.completed = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for completed: {}", e));
      self
    }
    pub fn id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<f64>,
      T::Error: std::fmt::Display,
    {
      self.id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for id: {}", e));
      self
    }
    pub fn title<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<String>,
      T::Error: std::fmt::Display,
    {
      self.title = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for title: {}", e));
      self
    }
    pub fn user_id<T>(mut self, value: T) -> Self
    where
      T: std::convert::TryInto<f64>,
      T::Error: std::fmt::Display,
    {
      self.user_id = value
        .try_into()
        .map_err(|e| format!("error converting supplied value for user_id: {}", e));
      self
    }
  }
  impl std::convert::TryFrom<Simple> for super::Simple {
    type Error = super::error::ConversionError;
    fn try_from(value: Simple) -> Result<Self, super::error::ConversionError> {
      Ok(Self {
        completed: value.completed?,
        id: value.id?,
        title: value.title?,
        user_id: value.user_id?,
      })
    }
  }
  impl From<super::Simple> for Simple {
    fn from(value: super::Simple) -> Self {
      Self {
        completed: Ok(value.completed),
        id: Ok(value.id),
        title: Ok(value.title),
        user_id: Ok(value.user_id),
      }
    }
  }
}
