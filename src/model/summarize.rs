use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{Cabin, DayOfWeek, Page, Pax, SortOrder};
use super::search::{ApiErrorBody, DEFAULT_SUMMARIZERS};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarizeInputs {
    pub filter: Value,
    pub page: Page,
    pub pax: Pax,
    pub slices: Vec<Value>,
    pub first_day_of_week: DayOfWeek,
    pub internal_user: bool,
    pub slice_index: u32,
    pub sorts: SortOrder,
    pub cabin: Cabin,
    pub max_legs_relative_to_min: u32,
    pub change_of_airport: bool,
    pub check_availability: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution: Option<String>,
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
            filter: serde_json::json!({}),
            page: Page::default(),
            pax: Pax::default(),
            slices: Vec::new(),
            first_day_of_week: DayOfWeek::Sunday,
            internal_user: false,
            slice_index: 0,
            sorts: SortOrder::Default,
            cabin: Cabin::Coach,
            max_legs_relative_to_min: 1,
            change_of_airport: true,
            check_availability: true,
            solution: None,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarizeResponse {
    #[serde(default)]
    pub error: Option<ApiErrorBody>,
    #[serde(flatten)]
    pub data: Value,
}
