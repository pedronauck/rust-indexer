use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::Bytes32;

// --------------------------------------------------------------------------------
// Helpers
// --------------------------------------------------------------------------------

pub type DynError = Box<dyn Error>;
pub type DynResult<T> = anyhow::Result<T>;
pub type AsyncDynResult<T> = anyhow::Result<T, Box<dyn Error + Send + Sync>>;

// --------------------------------------------------------------------------------
// HexString
// --------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct HexString(pub String);
impl From<Bytes32> for HexString {
    fn from(bytes: Bytes32) -> Self {
        Self(format!("{bytes:#x}"))
    }
}
impl std::fmt::Display for HexString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
