use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{
    Cabin, DateModifier, DayOfWeek, Page, Pax, SearchFilter, SearchName, SliceFilter,
};
use super::response::CarrierStopMatrix;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchInputs {
    pub filter: SearchFilter,
    pub page: Page,
    pub pax: Pax,
    pub slices: Vec<Slice>,
    pub first_day_of_week: DayOfWeek,
    pub internal_user: bool,
    pub slice_index: u32,
    pub sorts: String,
    pub cabin: Cabin,
    pub max_legs_relative_to_min: u32,
    pub change_of_airport: bool,
    pub check_availability: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layover: Option<Layover>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fare_keys: Option<String>,
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

pub const SUMMARIZERS_WHOLE_TRIP: &[&str] = &[
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

pub const SUMMARIZERS_BY_SLICE: &[&str] = &[
    "carrierStopMatrixSlice",
    "solutionListSlice",
    "stopCountListSlice",
    "departureTimeRangesSlice",
    "arrivalTimeRangesSlice",
    "currencyNotice",
    "durationSliderSlice",
    "originsSlice",
    "destinationsSlice",
    "warningsSlice",
    "priceSliderSlice",
];

pub const SUMMARIZERS_VIEW_DETAILS: &[&str] = &["bookingDetails"];
pub const SUMMARIZERS_VIEW_RULES: &[&str] = &["fareRules"];
pub const SUMMARIZERS_CALENDAR_RT: &[&str] = &["calendar", "currencyNotice"];
pub const SUMMARIZERS_CALENDAR_OW: &[&str] = &["calendarOneWay", "currencyNotice"];
pub const SUMMARIZERS_TIMEBAR: &[&str] = &["timebar", "currencyNotice"];
pub const SUMMARIZERS_TIMEBAR_OW: &[&str] = &["timebarsOneWay", "currencyNotice"];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub summarizers: Vec<String>,
    pub inputs: SearchInputs,
    pub summarizer_set: String,
    pub name: SearchName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg_program_response: Option<String>,
}

impl SearchRequest {
    pub fn builder() -> SearchRequestBuilder {
        SearchRequestBuilder::default()
    }

    /// Build a calendar search request.
    pub fn calendar(
        origins: &[&str],
        destinations: &[&str],
        start_date: &str,
        end_date: &str,
    ) -> Self {
        assert!(!origins.is_empty(), "origins required");
        assert!(!destinations.is_empty(), "destinations required");
        assert!(!start_date.is_empty(), "start_date required");
        assert!(!end_date.is_empty(), "end_date required");

        #[cfg(debug_assertions)]
        let start_date = dbg!(start_date);
        #[cfg(not(debug_assertions))]
        let start_date = start_date;

        let slice = calendar_slice(origins, destinations, start_date);
        let inputs = calendar_inputs(vec![slice], start_date, end_date);

        Self {
            summarizers: SUMMARIZERS_CALENDAR_OW
                .iter()
                .map(|s| s.to_string())
                .collect(),
            inputs,
            summarizer_set: "calendarOneWay".to_string(),
            name: SearchName::Calendar,
            bg_program_response: None,
        }
    }

    /// Build a round-trip calendar search request.
    pub fn calendar_round_trip(
        origins: &[&str],
        destinations: &[&str],
        start_date: &str,
        end_date: &str,
        return_start: &str,
        return_end: &str,
    ) -> Self {
        assert!(!origins.is_empty(), "origins required");
        assert!(!destinations.is_empty(), "destinations required");
        assert!(!start_date.is_empty(), "start_date required");
        assert!(!end_date.is_empty(), "end_date required");
        assert!(!return_start.is_empty(), "return_start required");
        assert!(!return_end.is_empty(), "return_end required");

        #[cfg(debug_assertions)]
        let return_end = dbg!(return_end);
        #[cfg(not(debug_assertions))]
        let return_end = return_end;

        let outbound = calendar_slice(origins, destinations, start_date);
        let inbound = calendar_slice(destinations, origins, return_start);
        let inputs = calendar_inputs(vec![outbound, inbound], start_date, end_date);

        let _ = return_end;

        Self {
            summarizers: SUMMARIZERS_CALENDAR_RT
                .iter()
                .map(|s| s.to_string())
                .collect(),
            inputs,
            summarizer_set: "calendarRoundTrip".to_string(),
            name: SearchName::Calendar,
            bg_program_response: None,
        }
    }
}

fn calendar_slice(origins: &[&str], destinations: &[&str], date: &str) -> Slice {
    Slice {
        origins: origins.iter().map(|s| s.to_string()).collect(),
        destinations: destinations.iter().map(|s| s.to_string()).collect(),
        date: date.to_string(),
        date_modifier: DateModifier::default(),
        is_arrival_date: false,
        filter: SliceFilter::default(),
        selected: false,
        route_language: None,
        command_line: None,
    }
}

fn calendar_inputs(slices: Vec<Slice>, start_date: &str, end_date: &str) -> SearchInputs {
    SearchInputs {
        filter: SearchFilter::default(),
        page: Page {
            current: 1,
            size: 25,
        },
        pax: Pax::default(),
        slices,
        first_day_of_week: DayOfWeek::Sunday,
        internal_user: false,
        slice_index: 0,
        sorts: "default".to_string(),
        cabin: Cabin::Coach,
        max_legs_relative_to_min: 1,
        change_of_airport: true,
        check_availability: true,
        start_date: Some(start_date.to_string()),
        end_date: Some(end_date.to_string()),
        layover: None,
        fare_keys: None,
    }
}

#[derive(Debug, Clone)]
struct SliceSpec {
    origins: Vec<String>,
    destinations: Vec<String>,
    date: String,
    route_language: Option<String>,
    command_line: Option<String>,
}

#[derive(Debug, Default)]
pub struct SearchRequestBuilder {
    origins: Vec<String>,
    destinations: Vec<String>,
    date: Option<String>,
    slices: Vec<SliceSpec>,
    cabin: Option<Cabin>,
    adults: Option<u8>,
    page_size: Option<u32>,
    max_legs_relative_to_min: Option<u32>,
    change_of_airport: Option<bool>,
    check_availability: Option<bool>,
    route_language: Option<String>,
    command_line: Option<String>,
    bg_program_response: Option<String>,
    sort: Option<String>,
}

impl SearchRequestBuilder {
    /// Add a slice to the request. Each call adds one flight leg.
    pub fn add_slice(mut self, origins: &[&str], destinations: &[&str], date: &str) -> Self {
        assert!(!origins.is_empty(), "origins required");
        assert!(!destinations.is_empty(), "destinations required");
        assert!(!date.is_empty(), "date required");

        self.slices.push(SliceSpec {
            origins: origins.iter().map(|s| s.to_string()).collect(),
            destinations: destinations.iter().map(|s| s.to_string()).collect(),
            date: date.to_string(),
            route_language: None,
            command_line: None,
        });
        self
    }

    /// Convenience: round-trip. Adds two slices.
    pub fn round_trip(
        self,
        origins: &[&str],
        destinations: &[&str],
        depart: &str,
        return_date: &str,
    ) -> Self {
        self.add_slice(origins, destinations, depart)
            .add_slice(destinations, origins, return_date)
    }

    /// Convenience: multi-city. Takes a slice of (origins, destinations, date) tuples.
    pub fn multi_city(mut self, legs: &[(&[&str], &[&str], &str)]) -> Self {
        for (origins, destinations, date) in legs {
            self = self.add_slice(origins, destinations, date);
        }
        self
    }

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

    pub fn command_line(mut self, cl: &str) -> Self {
        self.command_line = Some(cl.to_string());
        self
    }

    pub fn bg_program_response(mut self, token: &str) -> Self {
        self.bg_program_response = Some(token.to_string());
        self
    }

    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_string());
        self
    }

    pub fn build(self) -> SearchRequest {
        let slices = if self.slices.is_empty() {
            assert!(!self.origins.is_empty(), "origins required");
            assert!(!self.destinations.is_empty(), "destinations required");
            assert!(self.date.is_some(), "date required");
            let date = self.date.unwrap_or_default();

            vec![Slice {
                origins: self.origins,
                destinations: self.destinations,
                date,
                date_modifier: DateModifier::default(),
                is_arrival_date: false,
                filter: SliceFilter::default(),
                selected: false,
                route_language: self.route_language,
                command_line: self.command_line,
            }]
        } else {
            self.slices
                .into_iter()
                .map(|slice| Slice {
                    origins: slice.origins,
                    destinations: slice.destinations,
                    date: slice.date,
                    date_modifier: DateModifier::default(),
                    is_arrival_date: false,
                    filter: SliceFilter::default(),
                    selected: false,
                    route_language: slice.route_language,
                    command_line: slice.command_line,
                })
                .collect()
        };

        let inputs = SearchInputs {
            filter: SearchFilter::default(),
            page: Page {
                current: 1,
                size: self.page_size.unwrap_or(25),
            },
            pax: Pax {
                adults: self.adults.unwrap_or(1),
                children: 0,
                infants_in_lap: 0,
                infants_in_seat: 0,
                seniors: 0,
                youth: 0,
            },
            slices,
            first_day_of_week: DayOfWeek::Sunday,
            internal_user: false,
            slice_index: 0,
            sorts: self.sort.unwrap_or_else(|| "default".to_string()),
            cabin: self.cabin.unwrap_or(Cabin::Coach),
            max_legs_relative_to_min: self.max_legs_relative_to_min.unwrap_or(1),
            change_of_airport: self.change_of_airport.unwrap_or(true),
            check_availability: self.check_availability.unwrap_or(true),
            start_date: None,
            end_date: None,
            layover: None,
            fare_keys: None,
        };

        let name = if inputs.slices.len() > 1 {
            SearchName::SpecificDates
        } else {
            SearchName::SpecificDatesSlice
        };

        SearchRequest {
            summarizers: DEFAULT_SUMMARIZERS.iter().map(|s| s.to_string()).collect(),
            inputs,
            summarizer_set: "wholeTrip".to_string(),
            name,
            bg_program_response: self.bg_program_response,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layover {
    pub min: u32,
    pub max: u32,
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
    pub carrier_stop_matrix: Option<CarrierStopMatrix>,
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
    #[serde(default)]
    pub display_price: Option<String>,
    #[serde(default)]
    pub display_total: Option<String>,
    #[serde(flatten)]
    pub data: Value,
}
