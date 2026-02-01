use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub code: String,
    pub display_name: String,
    #[serde(default)]
    pub city_code: Option<String>,
    #[serde(default)]
    pub city_name: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub lat_lng: Option<LatLng>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub location_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatLng {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LocationsResponse {
    pub locations: Vec<Location>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub code: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurrenciesResponse {
    pub currencies: Vec<Currency>,
}
