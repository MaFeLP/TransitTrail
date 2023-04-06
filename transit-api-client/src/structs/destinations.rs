//!
//! Structures for the [destinations endpoint](crate::TransitClient::destinations)
//!

use serde::Deserialize;

/// An important landmark the buses on the variant will pass
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Destination {
    /// The key of the important landmark
    pub key: u32,

    /// The name of the important landmark
    pub name: String,
}
