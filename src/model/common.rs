use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Cabin {
    Coach,
    PremiumCoach,
    Business,
    First,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchName {
    #[serde(rename = "specificDatesSlice")]
    SpecificDatesSlice,
    #[serde(rename = "specificDates")]
    SpecificDates,
    #[serde(rename = "calendar")]
    Calendar,
    #[serde(rename = "calendarFollowup")]
    CalendarFollowup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pax {
    pub adults: u8,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub children: u8,
    #[serde(default, skip_serializing_if = "is_zero", rename = "infantsInLap")]
    pub infants_in_lap: u8,
    #[serde(default, skip_serializing_if = "is_zero", rename = "infantsInSeat")]
    pub infants_in_seat: u8,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub seniors: u8,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub youth: u8,
}

fn is_zero(v: &u8) -> bool {
    *v == 0
}

impl Default for Pax {
    fn default() -> Self {
        Self {
            adults: 1,
            children: 0,
            infants_in_lap: 0,
            infants_in_seat: 0,
            seniors: 0,
            youth: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub current: u32,
    pub size: u32,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            current: 1,
            size: 25,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct DateModifier {
    pub minus: u32,
    pub plus: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carriers: Option<CarrierFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_stop_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overnight: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceFilter>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CarrierFilter {
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub min: String,
    pub max: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SliceFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carriers: Option<CarrierFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_count: Option<CarrierFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arrival_time: Option<TimeRangeFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<TimeRangeFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<CodeFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<CodeFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<WarningsFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<DurationFilter>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeRangeFilter {
    pub ranges: Vec<TimeRange>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeFilter {
    pub codes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WarningsFilter {
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DurationFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<u32>,
}
