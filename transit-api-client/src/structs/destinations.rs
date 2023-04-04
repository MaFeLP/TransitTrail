use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Destination {
    pub key: u32,
    pub name: String,
}
