use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Regular {
    pub key: u32,
    pub number: u32,
    pub name: String,
    #[serde(rename = "customer-type")]
    pub customer_type: Customer,
    pub coverage: Coverage,
    #[serde(rename = "badge-label")]
    pub badge_label: u32,
    #[serde(rename = "badge-style")]
    pub badge_style: badges::Style,
    // Is always set on the 'routes' endpoint, but not set in the 'stops' endpoint
    pub variants: Option<Vec<Variant>>,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Blue {
    pub key: String,
    pub number: String,
    #[serde(rename = "customer-type")]
    pub customer_type: Customer,
    pub coverage: Coverage,
    #[serde(rename = "badge-label")]
    pub badge_label: String,
    #[serde(rename = "badge-style")]
    pub badge_style: badges::Style,
    pub variants: Option<Vec<Variant>>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Route {
    Blue(Blue),
    Regular(Regular),
}

impl Default for Route {
    fn default() -> Self {
        Self::Regular(Regular::default())
    }
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Customer {
    #[default]
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "industrial")]
    Industrial,
    #[serde(rename = "school")]
    School,
    #[serde(rename = "charter")]
    Charter,
    #[serde(rename = "work")]
    Work,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Coverage {
    #[default]
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "express")]
    Express,
    #[serde(rename = "super express")]
    SuperExpress,
    #[serde(rename = "rapid transit")]
    RapidTransit,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    pub key: String,
    pub name: Option<String>,
}

pub mod badges {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
    pub struct Style {
        #[serde(rename = "class-names")]
        pub class_names: ClassNames,
        #[serde(rename = "background-color")]
        pub background_color: String,
        #[serde(rename = "border-color")]
        pub border_color: String,
        pub color: String,
    }

    #[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
    pub struct ClassNames {
        #[serde(rename = "class-name")]
        pub class_name: Vec<String>,
    }
}