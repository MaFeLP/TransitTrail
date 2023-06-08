//!
//! A route is a named and numbered pattern of service that covers a certain geographic area with a
//! consistent method of service delivery.
//!

use serde::{Deserialize, Serialize};

/// Represents a NON-BLUE route.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Regular {
    /// The unique key of the route, most of the time the same as the number
    pub key: u32,

    /// The line number of the route
    pub number: u32,

    /// The name of the route, e.g. where it's going
    pub name: String,

    /// Who is buying the route
    #[serde(rename = "customer-type")]
    pub customer_type: Customer,

    /// If the route skips specific stops on the way
    pub coverage: Coverage,

    /// What is on the badge of the route
    #[serde(rename = "badge-label")]
    pub badge_label: u32,

    /// How this route's badge should be styled. For more info, see [badges]
    #[serde(rename = "badge-style")]
    pub badge_style: badges::Style,

    /// Variants of the current route, e.g. if the route splits up, where it's destination is.
    ///
    /// Is always set on the [routes](crate::endpoints::routes) endpoint, but not set in the
    /// [stops](crate::endpoints::stops) endpoint
    pub variants: Option<Vec<Variant>>,
}

/// Represents BLUE routes, as they have Strings as keys and numbers
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Blue {
    /// The unique key of the route, most of the time the same as the number
    pub key: String,

    /// The line number of the route
    pub number: String,

    /// Who is buying the route
    #[serde(rename = "customer-type")]
    pub customer_type: Customer,

    /// If the route skips specific stops on the way
    pub coverage: Coverage,

    /// What is on the badge of the route
    #[serde(rename = "badge-label")]
    pub badge_label: String,

    /// How this route's badge should be styled. For more info, see [badges]
    #[serde(rename = "badge-style")]
    pub badge_style: badges::Style,

    /// Variants of the current route, e.g. if the route splits up, where it's destination is.
    ///
    /// Is always set on the [routes](crate::endpoints::routes) endpoint, but not set in the
    /// [stops](crate::endpoints::stops) endpoint
    pub variants: Option<Vec<Variant>>,
}

/// Collects all types of routes. BLUE routes are rapid-transit lines and have
/// strings as their route number and key.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Route {
    /// The BLUE routes
    Blue(Blue),

    /// All routes that are not a BLUE line
    Regular(Regular),
}

impl Default for Route {
    fn default() -> Self {
        Self::Regular(Regular::default())
    }
}

/// The type of service provided by this route.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Customer {
    /// Regular Service at this route
    #[default]
    #[serde(rename = "regular")]
    Regular,

    /// Industrial Service
    #[serde(rename = "industrial")]
    Industrial,

    /// Service for specific schools
    #[serde(rename = "school")]
    School,

    /// Chartered Buses
    #[serde(rename = "charter")]
    Charter,

    /// Work buses
    #[serde(rename = "work")]
    Work,
}

/// Categorization of how fully a route services stops along it's segments.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Coverage {
    /// services all stops
    #[default]
    #[serde(rename = "regular")]
    Regular,

    /// services select stops in express segments
    #[serde(rename = "express")]
    Express,

    /// services no stops in express segments
    #[serde(rename = "super express")]
    SuperExpress,

    /// Special Type of coverage for BLUE routes
    #[serde(rename = "rapid transit")]
    RapidTransit,

    /// A feeder bus for express and rapid transit busses
    #[serde(rename = "feeder")]
    Feeder,

    /// A feeder bus for express and rapid transit busses, which is used at peak times
    #[serde(rename = "peak feeder")]
    PeakFeeder,
}

/// A variant is a variation of a route, distinguished by its intermediate destination points.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    ///  A unique identifier for this variant.
    pub key: String,

    /// The variant name.
    pub name: Option<String>,
}

pub mod badges {
    //! We will now be providing additional branding information for routes:
    //!
    //! Route badge styles will be provided for route results nested under the this module.
    //! This includes CSS colour codes for styling route badge labels - font colour, background
    //! colour, and border colour.
    //!

    use serde::{Deserialize, Serialize};

    /// How the route should be styler
    #[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
    pub struct Style {
        /// Additional classes to apply to nodes for styling
        #[serde(rename = "class-names")]
        pub class_names: ClassNames,
        /// The background colour of the badge
        #[serde(rename = "background-color")]
        pub background_color: String,
        /// The colour of the border of the badge
        #[serde(rename = "border-color")]
        pub border_color: String,
        /// The colour of the line
        pub color: String,
    }

    /// Additional class names are nested in here
    #[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
    pub struct ClassNames {
        /// Additional class names that should be applied
        #[serde(rename = "class-name")]
        pub class_name: Vec<String>,
    }
}
