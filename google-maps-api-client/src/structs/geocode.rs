use serde::{Deserialize, Serialize};

// Define your data structures for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct GeocodeResult {
    // Define the necessary fields for your use case
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeocodeResponse {
    results: Vec<GeocodeResult>,
    // Add other fields as needed
}
