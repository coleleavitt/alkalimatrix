use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{Cabin, DateModifier, DayOfWeek, Page, Pax, SliceFilter, SortOrder};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Slice {
    pub origins: Vec<String>,
    pub destinations: Vec<String>,
    pub date: String,
    #[serde(default)]
    pub date_modifier: DateModifier,
    pub is_arrival_date: bool,
    pub filter: SliceFilter,
    pub selected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_language: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchInputs {
    pub filter: Value,
    pub page: Page,
    pub pax: Pax,
    pub slices: Vec<Slice>,
    pub first_day_of_week: DayOfWeek,
    pub internal_user: bool,
    pub slice_index: u32,
    pub sorts: SortOrder,
    pub cabin: Cabin,
    pub max_legs_relative_to_min: u32,
    pub change_of_airport: bool,
    pub check_availability: bool,
}

pub const DEFAULT_SUMMARIZERS: &[&str] = &[
    "carrierStopMatrix",
    "currencyNotice",
    "solutionList",
    "itineraryPriceSlider",
    "itineraryCarrierList",
    "itineraryDepartureTimeRanges",
    "itineraryArrivalTimeRanges",
    "durationSliderItinerary",
    "itineraryOrigins",
    "itineraryDestinations",
    "itineraryStopCountList",
    "warningsItinerary",
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub summarizers: Vec<String>,
    pub inputs: SearchInputs,
    pub summarizer_set: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg_program_response: Option<String>,
}

impl SearchRequest {
    pub fn builder() -> SearchRequestBuilder {
        SearchRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SearchRequestBuilder {
    origins: Vec<String>,
    destinations: Vec<String>,
    date: Option<String>,
    cabin: Option<Cabin>,
    adults: Option<u8>,
    page_size: Option<u32>,
    max_legs_relative_to_min: Option<u32>,
    change_of_airport: Option<bool>,
    check_availability: Option<bool>,
    route_language: Option<String>,
    bg_program_response: Option<String>,
}

impl SearchRequestBuilder {
    pub fn origins(mut self, codes: &[&str]) -> Self {
        self.origins = codes.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn destinations(mut self, codes: &[&str]) -> Self {
        self.destinations = codes.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn date(mut self, date: &str) -> Self {
        self.date = Some(date.to_string());
        self
    }

    pub fn cabin(mut self, cabin: Cabin) -> Self {
        self.cabin = Some(cabin);
        self
    }

    pub fn adults(mut self, n: u8) -> Self {
        self.adults = Some(n);
        self
    }

    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }

    pub fn max_stops(mut self, n: u32) -> Self {
        self.max_legs_relative_to_min = Some(n);
        self
    }

    pub fn change_of_airport(mut self, allow: bool) -> Self {
        self.change_of_airport = Some(allow);
        self
    }

    pub fn check_availability(mut self, check: bool) -> Self {
        self.check_availability = Some(check);
        self
    }

    pub fn route_language(mut self, rl: &str) -> Self {
        self.route_language = Some(rl.to_string());
        self
    }

    pub fn bg_program_response(mut self, token: &str) -> Self {
        self.bg_program_response = Some(token.to_string());
        self
    }

    pub fn build(self) -> SearchRequest {
        assert!(!self.origins.is_empty(), "origins required");
        assert!(!self.destinations.is_empty(), "destinations required");
        let date = self.date.expect("date required");

        let slice = Slice {
            origins: self.origins,
            destinations: self.destinations,
            date,
            date_modifier: DateModifier::default(),
            is_arrival_date: false,
            filter: SliceFilter::default(),
            selected: false,
            route_language: self.route_language,
        };

        let inputs = SearchInputs {
            filter: serde_json::json!({}),
            page: Page {
                current: 1,
                size: self.page_size.unwrap_or(25),
            },
            pax: Pax {
                adults: self.adults.unwrap_or(1),
            },
            slices: vec![slice],
            first_day_of_week: DayOfWeek::Sunday,
            internal_user: false,
            slice_index: 0,
            sorts: SortOrder::Default,
            cabin: self.cabin.unwrap_or(Cabin::Coach),
            max_legs_relative_to_min: self.max_legs_relative_to_min.unwrap_or(1),
            change_of_airport: self.change_of_airport.unwrap_or(true),
            check_availability: self.check_availability.unwrap_or(true),
        };

        SearchRequest {
            summarizers: DEFAULT_SUMMARIZERS.iter().map(|s| s.to_string()).collect(),
            inputs,
            summarizer_set: "wholeTrip".to_string(),
            name: "specificDatesSlice".to_string(),
            bg_program_response: self.bg_program_response,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    #[serde(default)]
    pub solution_set: Option<String>,
    #[serde(default)]
    pub session: Option<String>,
    #[serde(default)]
    pub error: Option<ApiErrorBody>,
    #[serde(default)]
    pub carrier_stop_matrix: Option<Value>,
    #[serde(default)]
    pub currency_notice: Option<Value>,
    #[serde(default)]
    pub solution_list: Option<SolutionList>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolutionList {
    #[serde(default)]
    pub solutions: Vec<Solution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    pub id: String,
    #[serde(flatten)]
    pub data: Value,
}
