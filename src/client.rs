use tracing::info;

use crate::error::{ItaError, Result};
use crate::model::location::{CurrenciesResponse, Currency, Location, LocationsResponse};
use crate::model::search::{SearchRequest, SearchResponse};
use crate::model::summarize::{SummarizeRequest, SummarizeResponse};
use crate::transport::BatchTransport;

const DEFAULT_API_KEY: &str = "AIzaSyBH1mte6BdKzvf0c2mYprkyvfHCRWmfX7g";

pub struct ItaClient {
    transport: BatchTransport,
}

impl ItaClient {
    pub fn new() -> Result<Self> {
        Self::with_api_key(DEFAULT_API_KEY)
    }

    pub fn with_api_key(api_key: &str) -> Result<Self> {
        Ok(Self {
            transport: BatchTransport::new(api_key)?,
        })
    }

    pub async fn autocomplete(&self, query: &str, page_size: u32) -> Result<Vec<Location>> {
        assert!(!query.is_empty(), "query must not be empty");

        let path = format!(
            "/v1/locationTypes/CITIES_AND_AIRPORTS/partialNames/{query}/locations?pageSize={page_size}"
        );
        let resp: LocationsResponse = self.transport.get(&path).await?;
        info!(count = resp.locations.len(), "autocomplete results");
        Ok(resp.locations)
    }

    pub async fn currencies(&self) -> Result<Vec<Currency>> {
        let resp: CurrenciesResponse = self.transport.get("/v1/currencies").await?;
        info!(count = resp.currencies.len(), "currencies");
        Ok(resp.currencies)
    }

    pub async fn lookup_airport(&self, code: &str) -> Result<Location> {
        assert!(!code.is_empty(), "airport code must not be empty");

        let path = format!("/v1/locationTypes/airportOrMultiAirportCity/locationCodes/{code}");
        self.transport.get(&path).await
    }

    pub async fn search(&self, request: &SearchRequest) -> Result<SearchResponse> {
        let body = serde_json::to_string(request).map_err(ItaError::Json)?;
        let (status, json_str) = self.transport.post_raw("/v1/search", &body).await?;

        let resp: SearchResponse = serde_json::from_str(&json_str).map_err(ItaError::Json)?;

        if let Some(ref err) = resp.error {
            info!(
                message = %err.message,
                error_type = %err.error_type,
                "API returned error in search response"
            );
        }

        if let Some(ref ss) = resp.solution_set {
            info!(solution_set = %ss, "search session established");
        }

        if status != 200 {
            return Err(ItaError::UnexpectedStatus {
                outer: 200,
                inner: status,
            });
        }

        Ok(resp)
    }

    pub async fn summarize(&self, request: &SummarizeRequest) -> Result<SummarizeResponse> {
        let body = serde_json::to_string(request).map_err(ItaError::Json)?;
        self.transport.post("/v1/summarize", &body).await
    }
}
