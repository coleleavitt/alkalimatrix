use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Top-level booking details response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingDetails {
    #[serde(default)]
    pub ext: Option<BookingExt>,
    #[serde(default)]
    pub itinerary: Option<Itinerary>,
    #[serde(default)]
    pub tickets: Vec<Ticket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingExt {
    #[serde(default)]
    pub total_price: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub price_per_mile: Option<String>,
    /// Catch-all for extra fields
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Itinerary {
    #[serde(default)]
    pub distance: Option<Distance>,
    #[serde(default)]
    pub slices: Vec<ItinerarySlice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distance {
    #[serde(default)]
    pub units: Option<String>,
    #[serde(default)]
    pub value: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItinerarySlice {
    #[serde(default)]
    pub origin: Option<Airport>,
    #[serde(default)]
    pub destination: Option<Airport>,
    #[serde(default)]
    pub duration: Option<u32>,
    #[serde(default)]
    pub ext: Option<SliceExt>,
    #[serde(default)]
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceExt {
    #[serde(default)]
    pub warnings: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Airport {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub city: Option<City>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    #[serde(default)]
    pub carrier: Option<Carrier>,
    #[serde(default)]
    pub origin: Option<Airport>,
    #[serde(default)]
    pub destination: Option<Airport>,
    #[serde(default)]
    pub departure: Option<String>,
    #[serde(default)]
    pub arrival: Option<String>,
    #[serde(default)]
    pub duration: Option<u32>,
    #[serde(default)]
    pub flight_number: Option<String>,
    #[serde(default)]
    pub aircraft: Option<Aircraft>,
    #[serde(default)]
    pub booking_infos: Vec<BookingInfo>,
    #[serde(default)]
    pub connection: Option<Connection>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Carrier {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub short_name: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aircraft {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingInfo {
    #[serde(default)]
    pub booking_code: Option<String>,
    #[serde(default)]
    pub cabin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    #[serde(default)]
    pub duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    #[serde(default)]
    pub pricings: Vec<Pricing>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pricing {
    #[serde(default)]
    pub pax_count: Option<u32>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Carrier × stops matrix from search results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarrierStopMatrix {
    #[serde(default)]
    pub rows: Vec<CarrierStopRow>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarrierStopRow {
    #[serde(default)]
    pub carrier: Option<Carrier>,
    #[serde(default)]
    pub stops: Vec<StopCell>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopCell {
    #[serde(default)]
    pub count: Option<u32>,
    #[serde(default)]
    pub min_price: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}
