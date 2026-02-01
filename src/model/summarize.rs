use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{Cabin, DayOfWeek, Page, Pax, SearchFilter};
use super::response::BookingDetails;
use super::search::{ApiErrorBody, DEFAULT_SUMMARIZERS};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarizeInputs {
    pub filter: SearchFilter,
    pub page: Page,
    pub pax: Pax,
    pub slices: Vec<Value>,
    pub first_day_of_week: DayOfWeek,
    pub internal_user: bool,
    pub slice_index: u32,
    pub sorts: String,
    pub cabin: Cabin,
    pub max_legs_relative_to_min: u32,
    pub change_of_airport: bool,
    pub check_availability: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fare_keys: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarizeRequest {
    pub summarizers: Vec<String>,
    pub inputs: SummarizeInputs,
    pub summarizer_set: String,
    pub solution_set: String,
    pub session: String,
}

impl SummarizeRequest {
    pub fn new(solution_set: &str, session: &str) -> Self {
        let inputs = SummarizeInputs {
            filter: SearchFilter::default(),
            page: Page::default(),
            pax: Pax::default(),
            slices: Vec::new(),
            first_day_of_week: DayOfWeek::Sunday,
            internal_user: false,
            slice_index: 0,
            sorts: "default".to_string(),
            cabin: Cabin::Coach,
            max_legs_relative_to_min: 1,
            change_of_airport: true,
            check_availability: true,
            solution: None,
            fare_keys: None,
        };

        Self {
            summarizers: DEFAULT_SUMMARIZERS.iter().map(|s| s.to_string()).collect(),
            inputs,
            summarizer_set: "wholeTrip".to_string(),
            solution_set: solution_set.to_string(),
            session: session.to_string(),
        }
    }

    pub fn booking_details(solution_set: &str, session: &str, solution_id: &str) -> Self {
        let mut req = Self::new(solution_set, session);
        req.summarizers = vec!["bookingDetails".to_string()];
        req.summarizer_set = "viewDetails".to_string();
        req.inputs.solution = Some(solution_id.to_string());
        req
    }

    /// Build a fare rules request.
    ///
    /// `solution_id`: `"{solutionSet}/{solutionId}"` (same format as booking_details)
    /// `pricing_index`: zero-based index into the ticket's pricings array (usually 0)
    /// `slice_index`: zero-based slice index (0 for one-way, 0 or 1 for round-trip)
    pub fn fare_rules(
        solution_set: &str,
        session: &str,
        solution_id: &str,
        pricing_index: u32,
        slice_index: u32,
    ) -> Self {
        assert!(!solution_set.is_empty(), "solution_set required");
        assert!(!session.is_empty(), "session required");
        assert!(!solution_id.is_empty(), "solution_id required");

        let fare_keys = format!("{pricing_index}/{slice_index}");

        let mut req = Self::new(solution_set, session);
        req.summarizers = vec!["fareRules".to_string()];
        req.summarizer_set = "viewRules".to_string();
        req.inputs.solution = Some(solution_id.to_string());
        req.inputs.fare_keys = Some(fare_keys);
        req
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarizeResponse {
    #[serde(default)]
    pub error: Option<ApiErrorBody>,
    #[serde(default)]
    pub booking_details: Option<BookingDetails>,
    #[serde(default)]
    pub fare_rules: Option<Value>,
    #[serde(flatten)]
    pub data: Value,
}
