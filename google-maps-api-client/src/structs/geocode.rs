use serde::{Deserialize, Serialize};

// Define your data structures for API responses

/// Result of the Geocode API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeocodeResult {
    /// The address components
    pub address_components: Vec<AddressComponents>,
    
    /// The formatted address 
    pub formatted_address: String,
    
    /// The geometry of the address
    pub geometry: Geometry,
    
    /// The place ID
    pub place_id: String,
    
    /// The plus code
    pub plus_code: Option<PlusCode>,
    
    /// The types of the address
    pub types: Vec<String>,
}


/// Response of the Geocode API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeocodeResponse {
    
    /// The results of the API
    #[serde(rename = "results")]
    pub results: Vec<GeocodeResult>,
    
    /// The status of the API
    #[serde(rename = "status")]
    pub status: String,
}

/// Address components of the Geocode API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressComponents {
    
    /// The long name of the address
    #[serde(rename = "long_name")]
    pub long_name: String,
    
    /// The short name of the address
    #[serde(rename = "short_name")]
    pub short_name: String,
    
    /// The types of the address
    #[serde(rename = "types")]
    pub types: Vec<String>,
}

/// Geometry of the Geocode API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Geometry {
    
    /// The location of the address
    #[serde(rename = "location")]
    pub location: Location,
    
    /// The location type of the address
    #[serde(rename = "location_type")]
    pub location_type: String,
    
    /// The viewport of the address
    #[serde(rename = "viewport")]
    pub viewport: Viewport,
}

/// Location of the Geocode API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    
    /// The latitude of the address
    #[serde(rename = "lat")]
    pub lat: f64,
    
    /// The longitude of the address
    #[serde(rename = "lng")]
    pub lng: f64,
}

/// Viewport of the Geocode API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Viewport {
    /// The northeast location of the address
    #[serde(rename = "northeast")]
    pub northeast: Location,
    
    /// The southwest location of the address
    #[serde(rename = "southwest")]
    pub southwest: Location,
}

/// Plus code of the Geocode API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlusCode {
    
    /// The compound code of the address
    #[serde(rename = "compound_code")]
    pub compound_code: String,
    
    /// The global code of the address
    #[serde(rename = "global_code")]
    pub global_code: String,
}
