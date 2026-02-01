use std::fmt::{self, Display, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alliance {
    OneWorld,
    SkyTeam,
    StarAlliance,
}

impl Display for Alliance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneWorld => f.write_str("oneworld"),
            Self::SkyTeam => f.write_str("skyteam"),
            Self::StarAlliance => f.write_str("star-alliance"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CabinClass {
    First,
    Business,
    PremiumEconomy,
    Economy,
}

impl Display for CabinClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::First => f.write_char('1'),
            Self::Business => f.write_char('2'),
            Self::PremiumEconomy => f.write_str("pe"),
            Self::Economy => f.write_char('3'),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftCategory {
    Jet,
    Turboprop,
    Piston,
    Train,
    Helicopter,
    Amphibian,
    Surface,
}

impl Display for AircraftCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Jet => "JET",
            Self::Turboprop => "TURBOPROP",
            Self::Piston => "PISTON",
            Self::Train => "TRAIN",
            Self::Helicopter => "HELICOPTER",
            Self::Amphibian => "AMPHIBIAN",
            Self::Surface => "SURFACE",
        };
        f.write_str(s)
    }
}

pub mod aircraft {
    pub const AIRBUS_A320_FAMILY: &str = "32S";
    pub const AIRBUS_A320NEO: &str = "32N";
    pub const AIRBUS_A321NEO: &str = "32Q";
    pub const AIRBUS_A330: &str = "330";
    pub const AIRBUS_A350: &str = "350";
    pub const AIRBUS_A380: &str = "380";
    pub const BOEING_737: &str = "737";
    pub const BOEING_737_MAX: &str = "7M8";
    pub const BOEING_747: &str = "747";
    pub const BOEING_757: &str = "757";
    pub const BOEING_767: &str = "767";
    pub const BOEING_777: &str = "777";
    pub const BOEING_787: &str = "787";
    pub const EMBRAER_E175: &str = "E75";
    pub const EMBRAER_E190: &str = "E90";
    pub const BOMBARDIER_CRJ: &str = "CRJ";
    pub const DASH_8: &str = "DH8";
}

#[derive(Debug, Clone, Default)]
pub struct RoutingCode {
    parts: Vec<String>,
}

impl RoutingCode {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn nonstop(mut self) -> Self {
        self.parts.push("N".to_string());
        self
    }

    #[must_use]
    pub fn nonstop_on(mut self, carrier: &str) -> Self {
        debug_assert!(!carrier.is_empty(), "carrier must not be empty");
        self.parts.push(format!("N:{carrier}"));
        self
    }

    #[must_use]
    pub fn carrier(mut self, code: &str) -> Self {
        debug_assert!(!code.is_empty(), "carrier code must not be empty");
        self.parts.push(format!("C:{code}"));
        self
    }

    #[must_use]
    pub fn carrier_one_or_more(mut self, code: &str) -> Self {
        debug_assert!(!code.is_empty(), "carrier code must not be empty");
        self.parts.push(format!("C:{code}+"));
        self
    }

    #[must_use]
    pub fn carriers(mut self, codes: &[&str]) -> Self {
        debug_assert!(!codes.is_empty(), "carrier codes must not be empty");
        self.parts.push(codes.join(","));
        self
    }

    #[must_use]
    pub fn operating_carrier(mut self, code: &str) -> Self {
        debug_assert!(!code.is_empty(), "carrier code must not be empty");
        self.parts.push(format!("O:{code}"));
        self
    }

    #[must_use]
    pub fn connection(mut self) -> Self {
        self.parts.push("X".to_string());
        self
    }

    #[must_use]
    pub fn connection_at(mut self, airport: &str) -> Self {
        debug_assert!(!airport.is_empty(), "airport code must not be empty");
        self.parts.push(format!("X:{airport}"));
        self
    }

    #[must_use]
    pub fn connections_at(mut self, airports: &[&str]) -> Self {
        debug_assert!(!airports.is_empty(), "airport codes must not be empty");
        self.parts.push(airports.join(","));
        self
    }

    #[must_use]
    pub fn flight(mut self, carrier: &str, number: u32) -> Self {
        debug_assert!(!carrier.is_empty(), "carrier code must not be empty");
        self.parts.push(format!("F:{carrier}{number}"));
        self
    }

    #[must_use]
    pub fn any_flight(mut self) -> Self {
        self.parts.push("F".to_string());
        self
    }

    #[must_use]
    pub fn optional(mut self) -> Self {
        if let Some(last) = self.parts.last_mut() {
            last.push('?');
        }
        self
    }

    #[must_use]
    pub fn one_or_more(mut self) -> Self {
        if let Some(last) = self.parts.last_mut() {
            last.push('+');
        }
        self
    }

    #[must_use]
    pub fn zero_or_more(mut self) -> Self {
        if let Some(last) = self.parts.last_mut() {
            last.push('*');
        }
        self
    }

    #[must_use]
    pub fn exclude(mut self) -> Self {
        if let Some(last) = self.parts.last_mut() {
            last.insert(0, '~');
        }
        self
    }

    #[must_use]
    pub fn exclude_carrier(mut self, code: &str) -> Self {
        debug_assert!(!code.is_empty(), "carrier code must not be empty");
        self.parts.push(format!("~C:{code}"));
        self
    }

    #[must_use]
    pub fn exclude_carriers(mut self, codes: &[&str]) -> Self {
        debug_assert!(!codes.is_empty(), "carrier codes must not be empty");
        self.parts.push(format!("~{}", codes.join(",")));
        self
    }

    #[must_use]
    pub fn raw(mut self, code: &str) -> Self {
        self.parts.push(code.to_string());
        self
    }

    #[must_use]
    pub fn build(self) -> String {
        self.parts.join(" ")
    }
}

impl Display for RoutingCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.clone().build())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExtensionCode {
    parts: Vec<String>,
}

impl ExtensionCode {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn no_codeshare(mut self) -> Self {
        self.parts.push("-CODESHARE".to_string());
        self
    }

    #[must_use]
    pub fn max_stops(mut self, n: u32) -> Self {
        self.parts.push(format!("MAXSTOPS {n}"));
        self
    }

    #[must_use]
    pub fn max_duration(mut self, hours: u32, minutes: u32) -> Self {
        debug_assert!(minutes < 60, "minutes must be < 60");
        self.parts.push(format!("MAXDUR {hours}:{minutes:02}"));
        self
    }

    #[must_use]
    pub fn max_miles(mut self, miles: u32) -> Self {
        self.parts.push(format!("MAXMILES {miles}"));
        self
    }

    #[must_use]
    pub fn min_miles(mut self, miles: u32) -> Self {
        self.parts.push(format!("MINMILES {miles}"));
        self
    }

    #[must_use]
    pub fn min_connection(mut self, hours: u32, minutes: u32) -> Self {
        debug_assert!(minutes < 60, "minutes must be < 60");
        self.parts.push(format!("MINCONNECT {hours}:{minutes:02}"));
        self
    }

    #[must_use]
    pub fn max_connection(mut self, hours: u32, minutes: u32) -> Self {
        debug_assert!(minutes < 60, "minutes must be < 60");
        self.parts.push(format!("MAXCONNECT {hours}:{minutes:02}"));
        self
    }

    #[must_use]
    pub fn alliance(mut self, alliance: Alliance) -> Self {
        self.parts.push(format!("ALLIANCE {alliance}"));
        self
    }

    #[must_use]
    pub fn alliances(mut self, alliances: &[Alliance]) -> Self {
        debug_assert!(!alliances.is_empty(), "alliances must not be empty");
        let codes: Vec<String> = alliances.iter().map(ToString::to_string).collect();
        self.parts.push(format!("ALLIANCE {}", codes.join(" ")));
        self
    }

    #[must_use]
    pub fn airlines(mut self, codes: &[&str]) -> Self {
        debug_assert!(!codes.is_empty(), "airline codes must not be empty");
        self.parts.push(format!("AIRLINES {}", codes.join(" ")));
        self
    }

    #[must_use]
    pub fn exclude_airlines(mut self, codes: &[&str]) -> Self {
        debug_assert!(!codes.is_empty(), "airline codes must not be empty");
        self.parts.push(format!("-AIRLINES {}", codes.join(" ")));
        self
    }

    #[must_use]
    pub fn operating_airlines(mut self, codes: &[&str]) -> Self {
        debug_assert!(!codes.is_empty(), "airline codes must not be empty");
        self.parts.push(format!("OPAIRLINES {}", codes.join(" ")));
        self
    }

    #[must_use]
    pub fn exclude_operating_airlines(mut self, codes: &[&str]) -> Self {
        debug_assert!(!codes.is_empty(), "airline codes must not be empty");
        self.parts.push(format!("-OPAIRLINES {}", codes.join(" ")));
        self
    }

    #[must_use]
    pub fn exclude_cities(mut self, codes: &[&str]) -> Self {
        debug_assert!(!codes.is_empty(), "city codes must not be empty");
        self.parts.push(format!("-CITIES {}", codes.join(" ")));
        self
    }

    #[must_use]
    pub fn no_redeyes(mut self) -> Self {
        self.parts.push("-REDEYES".to_string());
        self
    }

    #[must_use]
    pub fn no_overnights(mut self) -> Self {
        self.parts.push("-OVERNIGHTS".to_string());
        self
    }

    #[must_use]
    pub fn aircraft_type(mut self, code: &str) -> Self {
        debug_assert!(!code.is_empty(), "aircraft code must not be empty");
        self.parts.push(format!("AIRCRAFT T:{code}"));
        self
    }

    #[must_use]
    pub fn aircraft_types(mut self, codes: &[&str]) -> Self {
        debug_assert!(!codes.is_empty(), "aircraft codes must not be empty");
        let formatted: Vec<String> = codes.iter().map(|c| format!("T:{c}")).collect();
        self.parts.push(format!("AIRCRAFT {}", formatted.join(" ")));
        self
    }

    #[must_use]
    pub fn aircraft_category(mut self, category: AircraftCategory) -> Self {
        self.parts.push(format!("AIRCRAFT C:{category}"));
        self
    }

    #[must_use]
    pub fn exclude_aircraft_type(mut self, code: &str) -> Self {
        debug_assert!(!code.is_empty(), "aircraft code must not be empty");
        self.parts.push(format!("-AIRCRAFT T:{code}"));
        self
    }

    #[must_use]
    pub fn no_props(mut self) -> Self {
        self.parts.push("-PROPS".to_string());
        self
    }

    #[must_use]
    pub fn require_first_class(mut self) -> Self {
        self.parts.push("-NOFIRSTCLASS".to_string());
        self
    }

    #[must_use]
    pub fn require_cabin(mut self, cabin: CabinClass) -> Self {
        self.parts.push(format!("+CABIN {cabin}"));
        self
    }

    #[must_use]
    pub fn exclude_cabin(mut self, cabin: CabinClass) -> Self {
        self.parts.push(format!("-CABIN {cabin}"));
        self
    }

    #[must_use]
    pub fn booking_class(mut self, code: &str) -> Self {
        debug_assert!(!code.is_empty(), "booking class must not be empty");
        self.parts.push(format!("F BC={code}"));
        self
    }

    #[must_use]
    pub fn fare_basis(mut self, basis: &str) -> Self {
        debug_assert!(!basis.is_empty(), "fare basis must not be empty");
        self.parts.push(format!("F ..{basis}"));
        self
    }

    #[must_use]
    pub fn raw(mut self, code: &str) -> Self {
        self.parts.push(code.to_string());
        self
    }

    #[must_use]
    pub fn build(self) -> String {
        self.parts.join(";")
    }
}

impl Display for ExtensionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.clone().build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routing_nonstop() {
        let code = RoutingCode::new().nonstop().build();
        assert_eq!(code, "N");
    }

    #[test]
    fn routing_carrier_one_or_more() {
        let code = RoutingCode::new().carrier_one_or_more("AA").build();
        assert_eq!(code, "C:AA+");
    }

    #[test]
    fn routing_exclude_carriers() {
        let code = RoutingCode::new()
            .exclude_carriers(&["AA", "UA", "DL"])
            .build();
        assert_eq!(code, "~AA,UA,DL");
    }

    #[test]
    fn routing_complex() {
        let code = RoutingCode::new()
            .carrier_one_or_more("AA")
            .connection_at("ORD")
            .carrier_one_or_more("UA")
            .build();
        assert_eq!(code, "C:AA+ X:ORD C:UA+");
    }

    #[test]
    fn routing_via_chicago_on_aa() {
        let code = RoutingCode::new()
            .carrier("AA")
            .connection_at("ORD")
            .carrier("AA")
            .build();
        assert_eq!(code, "C:AA X:ORD C:AA");
    }

    #[test]
    fn extension_basic() {
        let code = ExtensionCode::new().no_codeshare().max_stops(2).build();
        assert_eq!(code, "-CODESHARE;MAXSTOPS 2");
    }

    #[test]
    fn extension_duration() {
        let code = ExtensionCode::new().max_duration(6, 45).build();
        assert_eq!(code, "MAXDUR 6:45");
    }

    #[test]
    fn extension_alliance() {
        let code = ExtensionCode::new()
            .alliance(Alliance::StarAlliance)
            .build();
        assert_eq!(code, "ALLIANCE star-alliance");
    }

    #[test]
    fn extension_complex() {
        let code = ExtensionCode::new()
            .alliance(Alliance::OneWorld)
            .no_redeyes()
            .max_duration(8, 0)
            .no_props()
            .build();
        assert_eq!(code, "ALLIANCE oneworld;-REDEYES;MAXDUR 8:00;-PROPS");
    }

    #[test]
    fn extension_aircraft() {
        let code = ExtensionCode::new()
            .aircraft_types(&[aircraft::BOEING_787, aircraft::AIRBUS_A350])
            .build();
        assert_eq!(code, "AIRCRAFT T:787 T:350");
    }

    #[test]
    fn extension_cabin() {
        let code = ExtensionCode::new()
            .require_cabin(CabinClass::Business)
            .build();
        assert_eq!(code, "+CABIN 2");
    }
}
