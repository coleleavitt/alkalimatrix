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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortOrder {
    Default,
    Price,
    Duration,
    Departure,
    Arrival,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pax {
    pub adults: u8,
}

impl Default for Pax {
    fn default() -> Self {
        Self { adults: 1 }
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


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct WarningsFilter {
    pub values: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SliceFilter {
    pub warnings: WarningsFilter,
}

