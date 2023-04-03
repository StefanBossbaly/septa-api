use serde_derive::Deserialize;

pub enum TransportType {
    Bus,
    RegionalRail,
    Nhsl,
    Subway,
    Trolley,
}

#[derive(Debug, Deserialize)]
pub enum RegionalRailsLine {
    Airport,
    ChestnutHillEast,
    ChestnutHillWest,
    Cynwyd,
    FoxChase,
    LansdaleDoylestown,
    MediaWawa,
    ManayunkNorristown,
    PaoliThorndale,
    Trenton,
    Warminster,
    WilmingtonNewark,
    WestTrenton,
    GlensideCombined,
    CenterCity,
}

impl RegionalRailsLine {
    pub fn aberration(&self) -> &'static str {
        match *self {
            Self::Airport => "AIR",
            Self::ChestnutHillEast => "CHE",
            Self::ChestnutHillWest => "CHW",
            Self::Cynwyd => "CYN",
            Self::FoxChase => "FOX",
            Self::LansdaleDoylestown => "LAN",
            Self::MediaWawa => "MED",
            Self::ManayunkNorristown => "NOR",
            Self::PaoliThorndale => "PAO",
            Self::Trenton => "TRE",
            Self::Warminster => "WAR",
            Self::WilmingtonNewark => "WIL",
            Self::WestTrenton => "WTR",
            Self::GlensideCombined => "GLN",
            Self::CenterCity => "CC",
        }
    }

    pub fn stops(&self) -> Vec<RegionalRailStop> {
        match *self {
            _ => unimplemented!(),
        }
    }
}

impl std::str::FromStr for RegionalRailsLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Airport" => Ok(Self::Airport),
            "Chestnut Hill East" => Ok(Self::ChestnutHillEast),
            "Chestnut Hill West" => Ok(Self::ChestnutHillWest),
            "Cynwyd" => Ok(Self::Cynwyd),
            "Fox Chase" => Ok(Self::FoxChase),
            "Lansdale/Doylestown" => Ok(Self::LansdaleDoylestown),
            "Media/Wawa" => Ok(Self::MediaWawa),
            "Manayunk/Norristown" => Ok(Self::ManayunkNorristown),
            "Paoli/Thorndale" => Ok(Self::PaoliThorndale),
            "Trenton" => Ok(Self::Trenton),
            "Warminster" => Ok(Self::Warminster),
            "Wilmington/Newark" => Ok(Self::WilmingtonNewark),
            "West Trenton" => Ok(Self::WestTrenton),
            "Glenside Combined" => Ok(Self::GlensideCombined),
            "Center City" => Ok(Self::CenterCity),
            _ => Err("Bad Value".to_string()),
        }
    }
}

impl std::fmt::Display for RegionalRailsLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Airport => write!(f, "Airport"),
            Self::ChestnutHillEast => write!(f, "Chestnut Hill East"),
            Self::ChestnutHillWest => write!(f, "Chestnut Hill West"),
            Self::Cynwyd => write!(f, "Cynwyd"),
            Self::FoxChase => write!(f, "Fox Chase"),
            Self::LansdaleDoylestown => write!(f, "Lansdale/Doylestown"),
            Self::MediaWawa => write!(f, "Media/Wawa"),
            Self::ManayunkNorristown => write!(f, "Manayunk/Norristown"),
            Self::PaoliThorndale => write!(f, "Paoli/Thorndale"),
            Self::Trenton => write!(f, "Trenton"),
            Self::Warminster => write!(f, "Warminster"),
            Self::WilmingtonNewark => write!(f, "Wilmington/Newark"),
            Self::WestTrenton => write!(f, "West Trenton"),
            Self::GlensideCombined => write!(f, "Glenside Combined"),
            Self::CenterCity => write!(f, "Center City"),
        }
    }
}

pub enum RegionalRailStop {
    // Chestnut Hill East Line Stops
    Stenton,
    Wyndmoor,
    Wister,
    Gravers,
    Germantown,
    Sedgwick,
    WayneJunction,
    ChestnutHillEast,
    WashingtonLane,
    MountAiry,

    // Chestnut Hill West Line Stops
    NorthPhiladelphia,
    Upsal,
    StMartins,
    ChestnutHillWest,
    CheltenAvenue,
    Carpenter,
    RichardAllenLane,
    Tulpehocken,
    Highland,
    QueenLane,

    // Paoli/Thorndale Line Stops
    Thorndale,
    Downingtown,
    Exton,
    Malvern,
    Paoli,
    Wayne,
    Berwyn,
    Villanova,
    Strafford,
    Daylesford,
    Radnor,
    BrynMawr,
    Haverford,
    Ardmore,
    Wynnewood,
    Narberth,
    Overbrook,
    Merion,

    // Shared Center City Stops
    Gray30thStreet,
    SuburbanStation,
    JeffersonStation,
    TempleUniversity,
}

impl RegionalRailStop {
    pub fn stop_id(&self) -> i32 {
        match *self {
            Self::Thorndale => 90501,
            Self::Downingtown => 90502,
            Self::Exton => 90504,
            Self::Malvern => 90505,
            Self::Paoli => 90506,
            Self::Wayne => 90511,
            Self::Berwyn => 90508,
            Self::Villanova => 90509,
            Self::Strafford => 90510,
            Self::Daylesford => 90507,
            Self::Radnor => 90513,
            Self::BrynMawr => 90516,
            Self::Haverford => 90517,
            Self::Ardmore => 90518,
            Self::Wynnewood => 90519,
            Self::Narberth => 90520,
            Self::Overbrook => 90522,
            Self::Merion => 90521,
            Self::SuburbanStation => 90005,
            Self::TempleUniversity => 90007,
            Self::Gray30thStreet => 90004,
            _ => unimplemented!(),
        }
    }
}
