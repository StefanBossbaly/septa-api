use std::{fmt, str::FromStr};

use crate::errors::Error;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use strum::{Display, EnumCount, EnumIter, EnumString};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumCount,
    EnumIter,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum TransportType {
    Bus,
    RegionalRail,
    Nhsl,
    Subway,
    Trolley,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumCount,
    EnumIter,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum ServiceType {
    Express,
    Local,

    #[strum(default)]
    Unknown(String),
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumCount,
    EnumIter,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[strum(serialize_all = "title_case", ascii_case_insensitive)]
pub enum RegionalRailsLine {
    Airport,
    ChestnutHillEast,
    ChestnutHillWest,
    Cynwyd,
    FoxChase,

    #[strum(serialize = "Lansdale/Doylestown")]
    LansdaleDoylestown,

    #[strum(serialize = "Media/Wawa")]
    MediaWawa,

    #[strum(serialize = "Manayunk/Norristown")]
    ManayunkNorristown,

    #[strum(serialize = "Paoli/Thorndale")]
    PaoliThorndale,
    Trenton,

    #[strum(serialize = "Warminster")]
    Warminster,

    #[strum(serialize = "Wilmington/Newark")]
    WilmingtonNewark,
    WestTrenton,
}

impl RegionalRailsLine {
    pub fn id(&self) -> &'static str {
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
        }
    }

    pub fn stops(&self) -> Vec<RegionalRailStop> {
        unimplemented!()
    }
}

#[derive(
    Clone, Debug, Display, EnumString, EnumCount, EnumIter, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[strum(serialize_all = "title_case", ascii_case_insensitive)]
pub enum RegionalRailStop {
    // Airport Line Stops
    #[strum(
        serialize = "Airport Terminal E F",
        serialize = "Airport Terminal E-F",
        serialize = "Airport Terminal E & F",
        to_string = "Airport Terminal E & F"
    )]
    AirportTerminalEF,
    #[strum(
        serialize = "Airport Terminal C D",
        serialize = "Airport Terminal C-D",
        serialize = "Airport Terminal C & D",
        to_string = "Airport Terminal C & D"
    )]
    AirportTerminalCD,
    #[strum(serialize = "Airport Terminal B")]
    AirportTerminalB,
    #[strum(serialize = "Airport Terminal A")]
    AirportTerminalA,
    Eastwick,

    // Chestnut Hill East Line Stops
    Stenton,
    Wyndmoor,
    Wister,
    Gravers,
    Germantown,
    Sedgwick,
    #[strum(serialize = "Chestnut Hill East", serialize = "Chestnut H East")]
    ChestnutHillEast,
    WashingtonLane,
    MountAiry,

    // Chestnut Hill West Line Stops
    NorthPhiladelphiaSepta,
    Upsal,
    #[strum(serialize = "St. Martins")]
    StMartins,
    #[strum(serialize = "Chestnut Hill West", serialize = "Chestnut H West")]
    ChestnutHillWest,
    CheltenAvenue,
    Carpenter,
    RichardAllenLane,
    Tulpehocken,
    Highland,
    QueenLane,

    // Cynwyd Line Stops
    Cynwyd,
    Bala,
    WynnefieldAvenue,

    // Fox Chase Line Stops
    FoxChase,
    Ryers,
    Cheltenham,
    Lawndale,
    Olney,

    // Lansdale/Doylestown Line Stops
    Doylestown,
    DelawareValleyUniversity,
    NewBritain,
    Chalfont,
    LinkBelt,
    Colmar,
    Fortuna,
    #[strum(
        serialize = "9th Street Lansdale",
        serialize = "9th St Lansdale",
        to_string = "9th St Lansdale"
    )]
    NinthStreetLansdale,
    Lansdale,
    Pennbrook,
    NorthWales,
    GwyneddValley,
    Penllyn,
    Ambler,
    FortWashington,
    Oreland,
    NorthHills,
    NorthBroad,

    // Media/Wawa Line Stops
    Wawa,
    #[strum(serialize = "Elwyn", serialize = "Elwyn Station", to_string = "Elwyn")]
    Elwyn,
    Media,
    #[strum(serialize = "Moylan-Rose Valley")]
    MoylanRoseValley,
    Wallingford,
    Swarthmore,
    #[strum(serialize = "Morton-Rutledge")]
    MortonRutledge,
    Secane,
    Primos,
    #[strum(serialize = "Clifton-Aldan")]
    CliftonAldan,
    Gladstone,
    Lansdowne,
    #[strum(serialize = "Fernwood-Yeadon", serialize = "Fernwood")]
    FernwoodYeadon,
    Angora,
    #[strum(
        serialize = "49th Street",
        serialize = "49th St",
        to_string = "49th St"
    )]
    FortyNinthStreet,

    // Manayunk/Norristown Line Stops
    #[strum(
        serialize = "Norristown - Elm Street",
        serialize = "Norristown Elm Street",
        to_string = "Norristown Elm Street"
    )]
    NorristownElmStreet,
    MainStreet,
    #[strum(
        serialize = "Norristown T.C.",
        serialize = "Norristown Transit Center",
        to_string = "Norristown Transit Center"
    )]
    NorristownTransitCenter,
    Conshohocken,
    SpringMill,
    Miquon,
    IvyRidge,
    Manayunk,
    Wissahickon,
    EastFalls,
    Allegheny,

    // Paoli/Thorndale Line Stops
    Thorndale,
    Downingtown,
    Whitford,
    Exton,
    Malvern,
    Paoli,
    Wayne,
    #[strum(serialize = "St. Davids")]
    StDavids,
    Berwyn,
    Devon,
    Villanova,
    Rosemont,
    BrynMawr,
    Strafford,
    Daylesford,
    Radnor,
    Haverford,
    Ardmore,
    Wynnewood,
    Narberth,
    Overbrook,
    Merion,

    // Trenton Line Stops
    Trenton,
    Levittown,
    Bristol,
    Croydon,
    Eddington,
    CornwellsHeights,
    Torresdale,
    HolmesburgJunction,
    Tacony,
    Bridesburg,
    NorthPhiladelphiaAmtrak,

    // Warminster Line Stops
    Warminster,
    Hatboro,
    WillowGrove,
    Crestmont,
    Roslyn,
    Ardsley,

    // Wilmington/Newark Line Stops
    #[strum(serialize = "Newark DE", serialize = "Newark", to_string = "Newark DE")]
    Newark,
    #[strum(serialize = "Churchman's Crossing")]
    ChurchmansCrossing,
    Wilmington,
    Claymont,
    MarcusHook,
    HighlandAvenue,
    #[strum(serialize = "Chester T.C.")]
    Chester,
    Eddystone,
    CrumLynne,
    RidleyPark,

    #[strum(
        serialize = "Prospect Park - Moore",
        serialize = "Prospect Park Moore",
        to_string = "Prospect Park - Moore"
    )]
    ProspectParkMoore,
    Norwood,
    Glenolden,
    Folcroft,
    SharonHill,
    CurtisPark,
    Darby,

    // West Trenton Line Stops
    WestTrenton,
    Yardley,
    Woodbourne,
    Langhorne,
    NeshaminyFalls,
    Trevose,
    Somerton,
    ForestHills,
    Philmont,
    Bethayres,
    Meadowbrook,
    Rydal,
    Noble,

    // Shared Stops
    WayneJunction,
    Glenside,
    #[strum(
        serialize = "Jenkintown-Wyncote",
        serialize = "Jenkintown Wyncote",
        to_string = "Jenkintown-Wyncote"
    )]
    JenkintownWyncote,
    #[strum(serialize = "Fern Rock Transit Center")]
    FernRockTC,
    ElkinsPark,
    MelrosePark,

    // Shared Center City Stops
    #[strum(
        serialize = "30th Street Station",
        serialize = "30th St",
        serialize = "30th Street Gray",
        serialize = "Gray 30th St",
        serialize = "Gray 30th St Station",
        to_string = "Gray 30th St Station"
    )]
    Gray30thStreet,
    SuburbanStation,
    #[strum(serialize = "Jefferson Station", serialize = "Jefferson")]
    JeffersonStation,
    #[strum(serialize = "Temple U", serialize = "Temple University")]
    TempleUniversity,
    #[strum(
        serialize = "Penn Medicine Station",
        serialize = "Penn Medical Station"
    )]
    PennMedicineStation,

    // Unknown Stop
    #[strum(default)]
    Unknown(String),
}

struct RegionalRailStopVisitor;

impl<'de> Visitor<'de> for RegionalRailStopVisitor {
    type Value = RegionalRailStop;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expecting a serialized SEPTA regional rail stop name")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match RegionalRailStop::from_str(value) {
            Ok(stop) => match stop {
                RegionalRailStop::Unknown(stop_name) => Err(E::custom(format!(
                    "Regional rail stop \"{}\" was not reconized",
                    stop_name
                ))),
                _ => Ok(stop),
            },
            Err(e) => Err(E::custom(format!(
                "Regional rail stop was not reconized becasue of error: {}",
                e
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for RegionalRailStop {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RegionalRailStopVisitor)
    }
}

impl RegionalRailStop {
    pub fn stop_id(&self) -> Result<u32, Error> {
        match *self {
            Self::Cynwyd => Ok(90001),
            Self::Bala => Ok(90002),
            Self::WynnefieldAvenue => Ok(90003),
            Self::Gray30thStreet => Ok(90004),
            Self::SuburbanStation => Ok(90005),
            Self::JeffersonStation => Ok(90006),
            Self::TempleUniversity => Ok(90007),
            Self::NorthBroad => Ok(90008),
            Self::WayneJunction => Ok(90009),
            Self::Newark => Ok(90201),
            Self::ChurchmansCrossing => Ok(90202),
            Self::Wilmington => Ok(90203),
            Self::Claymont => Ok(90204),
            Self::MarcusHook => Ok(90205),
            Self::HighlandAvenue => Ok(90206),
            Self::Chester => Ok(90207),
            Self::Eddystone => Ok(90208),
            Self::CrumLynne => Ok(90209),
            Self::RidleyPark => Ok(90210),
            Self::ProspectParkMoore => Ok(90211),
            Self::Norwood => Ok(90212),
            Self::Glenolden => Ok(90213),
            Self::Folcroft => Ok(90214),
            Self::SharonHill => Ok(90215),
            Self::CurtisPark => Ok(90216),
            Self::Darby => Ok(90217),
            Self::Allegheny => Ok(90218),
            Self::EastFalls => Ok(90219),
            Self::Wissahickon => Ok(90220),
            Self::Manayunk => Ok(90221),
            Self::IvyRidge => Ok(90222),
            Self::Miquon => Ok(90223),
            Self::SpringMill => Ok(90224),
            Self::Conshohocken => Ok(90225),
            Self::NorristownTransitCenter => Ok(90226),
            Self::MainStreet => Ok(90227),
            Self::NorristownElmStreet => Ok(90228),
            Self::Wawa => Ok(90300),
            Self::Elwyn => Ok(90301),
            Self::Media => Ok(90302),
            Self::MoylanRoseValley => Ok(90303),
            Self::Wallingford => Ok(90304),
            Self::Swarthmore => Ok(90305),
            Self::MortonRutledge => Ok(90306),
            Self::Secane => Ok(90307),
            Self::Primos => Ok(90308),
            Self::CliftonAldan => Ok(90309),
            Self::Gladstone => Ok(90310),
            Self::Lansdowne => Ok(90311),
            Self::FernwoodYeadon => Ok(90312),
            Self::Angora => Ok(90313),
            Self::FortyNinthStreet => Ok(90314),
            Self::Noble => Ok(90315),
            Self::Rydal => Ok(90316),
            Self::Meadowbrook => Ok(90317),
            Self::Bethayres => Ok(90318),
            Self::Philmont => Ok(90319),
            Self::ForestHills => Ok(90320),
            Self::Somerton => Ok(90321),
            Self::Trevose => Ok(90322),
            Self::NeshaminyFalls => Ok(90323),
            Self::Langhorne => Ok(90324),
            Self::Woodbourne => Ok(90325),
            Self::Yardley => Ok(90326),
            Self::WestTrenton => Ok(90327),
            Self::AirportTerminalEF => Ok(90401),
            Self::AirportTerminalCD => Ok(90402),
            Self::AirportTerminalB => Ok(90403),
            Self::AirportTerminalA => Ok(90404),
            Self::Eastwick => Ok(90405),
            Self::PennMedicineStation => Ok(90406),
            Self::FernRockTC => Ok(90407),
            Self::MelrosePark => Ok(90408),
            Self::ElkinsPark => Ok(90409),
            Self::JenkintownWyncote => Ok(90410),
            Self::Glenside => Ok(90411),
            Self::Ardsley => Ok(90412),
            Self::Roslyn => Ok(90413),
            Self::Crestmont => Ok(90414),
            Self::WillowGrove => Ok(90415),
            Self::Hatboro => Ok(90416),
            Self::Warminster => Ok(90417),
            Self::Thorndale => Ok(90501),
            Self::Downingtown => Ok(90502),
            Self::Whitford => Ok(90503),
            Self::Exton => Ok(90504),
            Self::Malvern => Ok(90505),
            Self::Paoli => Ok(90506),
            Self::Daylesford => Ok(90507),
            Self::Berwyn => Ok(90508),
            Self::Devon => Ok(90509),
            Self::Strafford => Ok(90510),
            Self::Wayne => Ok(90511),
            Self::StDavids => Ok(90512),
            Self::Radnor => Ok(90513),
            Self::Villanova => Ok(90514),
            Self::Rosemont => Ok(90515),
            Self::BrynMawr => Ok(90516),
            Self::Haverford => Ok(90517),
            Self::Ardmore => Ok(90518),
            Self::Wynnewood => Ok(90519),
            Self::Narberth => Ok(90520),
            Self::Merion => Ok(90521),
            Self::Overbrook => Ok(90522),
            Self::NorthHills => Ok(90523),
            Self::Oreland => Ok(90524),
            Self::FortWashington => Ok(90525),
            Self::Ambler => Ok(90526),
            Self::Penllyn => Ok(90527),
            Self::GwyneddValley => Ok(90528),
            Self::NorthWales => Ok(90529),
            Self::Pennbrook => Ok(90530),
            Self::Lansdale => Ok(90531),
            Self::Fortuna => Ok(90532),
            Self::Colmar => Ok(90533),
            Self::LinkBelt => Ok(90534),
            Self::Chalfont => Ok(90535),
            Self::NewBritain => Ok(90536),
            Self::DelawareValleyUniversity => Ok(90537),
            Self::Doylestown => Ok(90538),
            Self::NinthStreetLansdale => Ok(90539),
            Self::Trenton => Ok(90701),
            Self::Levittown => Ok(90702),
            Self::Bristol => Ok(90703),
            Self::Croydon => Ok(90704),
            Self::Eddington => Ok(90705),
            Self::CornwellsHeights => Ok(90706),
            Self::Torresdale => Ok(90707),
            Self::HolmesburgJunction => Ok(90708),
            Self::Tacony => Ok(90709),
            Self::Bridesburg => Ok(90710),
            Self::NorthPhiladelphiaAmtrak => Ok(90711),
            Self::Wister => Ok(90712),
            Self::Germantown => Ok(90713),
            Self::WashingtonLane => Ok(90714),
            Self::Stenton => Ok(90715),
            Self::Sedgwick => Ok(90716),
            Self::MountAiry => Ok(90717),
            Self::Wyndmoor => Ok(90718),
            Self::Gravers => Ok(90719),
            Self::ChestnutHillEast => Ok(90720),
            Self::ChestnutHillWest => Ok(90801),
            Self::Highland => Ok(90802),
            Self::StMartins => Ok(90803),
            Self::RichardAllenLane => Ok(90804),
            Self::Carpenter => Ok(90805),
            Self::Upsal => Ok(90806),
            Self::Tulpehocken => Ok(90807),
            Self::CheltenAvenue => Ok(90808),
            Self::QueenLane => Ok(90809),
            Self::NorthPhiladelphiaSepta => Ok(90810),
            Self::Olney => Ok(90811),
            Self::Lawndale => Ok(90812),
            Self::Cheltenham => Ok(90813),
            Self::Ryers => Ok(90814),
            Self::FoxChase => Ok(90815),
            Self::Unknown(ref station) => {
                Err(Error::UnknownRegionalRailStation(station.to_string()))
            }
        }
    }

    pub fn lat_lon(&self) -> Result<(f64, f64), Error> {
        match *self {
            Self::Cynwyd => Ok((40.006670, -75.231670)),
            Self::Bala => Ok((40.001110, -75.227780)),
            Self::WynnefieldAvenue => Ok((39.990000, -75.225560)),
            Self::Gray30thStreet => Ok((39.956670, -75.181660)),
            Self::SuburbanStation => Ok((39.953890, -75.167780)),
            Self::JeffersonStation => Ok((39.952500, -75.158060)),
            Self::TempleUniversity => Ok((39.981390, -75.149440)),
            Self::NorthBroad => Ok((39.992220, -75.153890)),
            Self::WayneJunction => Ok((40.022220, -75.160000)),
            Self::Newark => Ok((39.669690, -75.753510)),
            Self::ChurchmansCrossing => Ok((39.695000, -75.672500)),
            Self::Wilmington => Ok((39.737260, -75.551090)),
            Self::Claymont => Ok((39.797780, -75.452220)),
            Self::MarcusHook => Ok((39.821670, -75.419440)),
            Self::HighlandAvenue => Ok((39.833610, -75.393330)),
            Self::Chester => Ok((39.849720, -75.360000)),
            Self::Eddystone => Ok((39.857220, -75.342220)),
            Self::CrumLynne => Ok((39.871940, -75.331110)),
            Self::RidleyPark => Ok((39.880550, -75.322220)),
            Self::ProspectParkMoore => Ok((39.888330, -75.308890)),
            Self::Norwood => Ok((39.891670, -75.301670)),
            Self::Glenolden => Ok((39.896390, -75.290000)),
            Self::Folcroft => Ok((39.900550, -75.279720)),
            Self::SharonHill => Ok((39.904450, -75.270840)),
            Self::CurtisPark => Ok((39.908050, -75.265000)),
            Self::Darby => Ok((39.913060, -75.254450)),
            Self::Allegheny => Ok((40.003610, -75.164720)),
            Self::EastFalls => Ok((40.011390, -75.191950)),
            Self::Wissahickon => Ok((40.016670, -75.210280)),
            Self::Manayunk => Ok((40.026940, -75.225000)),
            Self::IvyRidge => Ok((40.034170, -75.235560)),
            Self::Miquon => Ok((40.058610, -75.266390)),
            Self::SpringMill => Ok((40.074170, -75.286110)),
            Self::Conshohocken => Ok((40.072220, -75.308610)),
            Self::NorristownTransitCenter => Ok((40.112780, -75.344170)),
            Self::MainStreet => Ok((40.117220, -75.348610)),
            Self::NorristownElmStreet => Ok((40.120830, -75.345000)),
            Self::Wawa => Ok((39.900680, -75.458560)),
            Self::Elwyn => Ok((39.907500, -75.411670)),
            Self::Media => Ok((39.914440, -75.395000)),
            Self::MoylanRoseValley => Ok((39.906110, -75.388610)),
            Self::Wallingford => Ok((39.903610, -75.371940)),
            Self::Swarthmore => Ok((39.902220, -75.350830)),
            Self::MortonRutledge => Ok((39.907780, -75.328890)),
            Self::Secane => Ok((39.915830, -75.309720)),
            Self::Primos => Ok((39.921670, -75.298330)),
            Self::CliftonAldan => Ok((39.926670, -75.290280)),
            Self::Gladstone => Ok((39.932780, -75.282220)),
            Self::Lansdowne => Ok((39.937500, -75.270840)),
            Self::FernwoodYeadon => Ok((39.939720, -75.255840)),
            Self::Angora => Ok((39.944720, -75.238610)),
            Self::FortyNinthStreet => Ok((39.943610, -75.216670)),
            Self::Noble => Ok((40.104440, -75.124170)),
            Self::Rydal => Ok((40.107500, -75.110560)),
            Self::Meadowbrook => Ok((40.111390, -75.092500)),
            Self::Bethayres => Ok((40.116660, -75.068340)),
            Self::Philmont => Ok((40.121940, -75.043610)),
            Self::ForestHills => Ok((40.127780, -75.020550)),
            Self::Somerton => Ok((40.130550, -75.011950)),
            Self::Trevose => Ok((40.140280, -74.982500)),
            Self::NeshaminyFalls => Ok((40.146950, -74.961670)),
            Self::Langhorne => Ok((40.160830, -74.912500)),
            Self::Woodbourne => Ok((40.192500, -74.889170)),
            Self::Yardley => Ok((40.235280, -74.830560)),
            Self::WestTrenton => Ok((40.257780, -74.815280)),
            Self::AirportTerminalEF => Ok((39.879440, -75.239720)),
            Self::AirportTerminalCD => Ok((39.878060, -75.240000)),
            Self::AirportTerminalB => Ok((39.877220, -75.241390)),
            Self::AirportTerminalA => Ok((39.876110, -75.245280)),
            Self::Eastwick => Ok((39.892780, -75.243890)),
            Self::PennMedicineStation => Ok((39.948060, -75.190280)),
            Self::FernRockTC => Ok((40.040550, -75.134720)),
            Self::MelrosePark => Ok((40.059440, -75.129170)),
            Self::ElkinsPark => Ok((40.071390, -75.127780)),
            Self::JenkintownWyncote => Ok((40.092780, -75.137500)),
            Self::Glenside => Ok((40.101390, -75.153610)),
            Self::Ardsley => Ok((40.114170, -75.153050)),
            Self::Roslyn => Ok((40.120830, -75.134160)),
            Self::Crestmont => Ok((40.133340, -75.118610)),
            Self::WillowGrove => Ok((40.143890, -75.114170)),
            Self::Hatboro => Ok((40.176110, -75.102500)),
            Self::Warminster => Ok((40.195280, -75.089160)),
            Self::Thorndale => Ok((39.992780, -75.763610)),
            Self::Downingtown => Ok((40.002190, -75.710780)),
            Self::Whitford => Ok((40.014720, -75.638050)),
            Self::Exton => Ok((40.019290, -75.621710)),
            Self::Malvern => Ok((40.036390, -75.515560)),
            Self::Paoli => Ok((40.042760, -75.483760)),
            Self::Daylesford => Ok((40.043060, -75.460560)),
            Self::Berwyn => Ok((40.048050, -75.442220)),
            Self::Devon => Ok((40.047220, -75.422780)),
            Self::Strafford => Ok((40.049450, -75.403050)),
            Self::Wayne => Ok((40.045830, -75.386670)),
            Self::StDavids => Ok((40.043890, -75.372500)),
            Self::Radnor => Ok((40.044720, -75.358890)),
            Self::Villanova => Ok((40.038330, -75.341670)),
            Self::Rosemont => Ok((40.027780, -75.326670)),
            Self::BrynMawr => Ok((40.021950, -75.316390)),
            Self::Haverford => Ok((40.013890, -75.299720)),
            Self::Ardmore => Ok((40.008280, -75.290400)),
            Self::Wynnewood => Ok((40.002780, -75.272500)),
            Self::Narberth => Ok((40.004720, -75.261390)),
            Self::Merion => Ok((39.998610, -75.251390)),
            Self::Overbrook => Ok((39.989440, -75.249440)),
            Self::NorthHills => Ok((40.111950, -75.169440)),
            Self::Oreland => Ok((40.118330, -75.183890)),
            Self::FortWashington => Ok((40.135830, -75.212220)),
            Self::Ambler => Ok((40.153610, -75.224720)),
            Self::Penllyn => Ok((40.170000, -75.244160)),
            Self::GwyneddValley => Ok((40.184720, -75.256940)),
            Self::NorthWales => Ok((40.214170, -75.277220)),
            Self::Pennbrook => Ok((40.230280, -75.281670)),
            Self::Lansdale => Ok((40.242780, -75.285000)),
            Self::Fortuna => Ok((40.259450, -75.266110)),
            Self::Colmar => Ok((40.268330, -75.254450)),
            Self::LinkBelt => Ok((40.273890, -75.246670)),
            Self::Chalfont => Ok((40.287780, -75.209720)),
            Self::NewBritain => Ok((40.297500, -75.179730)),
            Self::DelawareValleyUniversity => Ok((40.297220, -75.161670)),
            Self::Doylestown => Ok((40.306390, -75.130280)),
            Self::NinthStreetLansdale => Ok((40.250000, -75.279170)),
            Self::Trenton => Ok((40.218510, -74.753930)),
            Self::Levittown => Ok((40.140280, -74.816950)),
            Self::Bristol => Ok((40.104720, -74.854720)),
            Self::Croydon => Ok((40.093610, -74.906670)),
            Self::Eddington => Ok((40.083060, -74.933610)),
            Self::CornwellsHeights => Ok((40.070900, -74.954320)),
            Self::Torresdale => Ok((40.054440, -74.984440)),
            Self::HolmesburgJunction => Ok((40.032780, -75.023610)),
            Self::Tacony => Ok((40.023330, -75.038890)),
            Self::Bridesburg => Ok((40.010560, -75.069730)),
            Self::NorthPhiladelphiaAmtrak => Ok((39.996780, -75.155110)),
            Self::Wister => Ok((40.036110, -75.161110)),
            Self::Germantown => Ok((40.037500, -75.171670)),
            Self::WashingtonLane => Ok((40.050830, -75.171390)),
            Self::Stenton => Ok((40.060550, -75.178610)),
            Self::Sedgwick => Ok((40.062780, -75.185280)),
            Self::MountAiry => Ok((40.065280, -75.190830)),
            Self::Wyndmoor => Ok((40.073330, -75.196660)),
            Self::Gravers => Ok((40.077500, -75.201670)),
            Self::ChestnutHillEast => Ok((40.081110, -75.207220)),
            Self::ChestnutHillWest => Ok((40.076390, -75.208340)),
            Self::Highland => Ok((40.070560, -75.211110)),
            Self::StMartins => Ok((40.065830, -75.204440)),
            Self::RichardAllenLane => Ok((40.057500, -75.194730)),
            Self::Carpenter => Ok((40.051110, -75.191390)),
            Self::Upsal => Ok((40.042500, -75.190000)),
            Self::Tulpehocken => Ok((40.035280, -75.186940)),
            Self::CheltenAvenue => Ok((40.030000, -75.180830)),
            Self::QueenLane => Ok((40.023330, -75.178050)),
            Self::NorthPhiladelphiaSepta => Ok((39.997780, -75.156390)),
            Self::Olney => Ok((40.033330, -75.122780)),
            Self::Lawndale => Ok((40.051390, -75.103060)),
            Self::Cheltenham => Ok((40.058060, -75.092780)),
            Self::Ryers => Ok((40.064170, -75.086390)),
            Self::FoxChase => Ok((40.076390, -75.083340)),
            Self::Unknown(ref station) => {
                Err(Error::UnknownRegionalRailStation(station.to_string()))
            }
        }
    }
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumCount,
    EnumIter,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum NhslStop {
    #[strum(serialize = "Norristown Transportation Center - NHSL")]
    NorristownTransportationCenter,
    #[strum(serialize = "Bridgeport Station - NHSL")]
    BridgeportStation,
    #[strum(serialize = "DeKalb St Station - NHSL")]
    DeKalbStStation,
    #[strum(serialize = "Hughes Park Station - NHSL")]
    HughesParkStation,
    #[strum(serialize = "Gulph Mills Station - NHSL")]
    GulphMillsStation,
    #[strum(serialize = "Matsonford Station - NHSL")]
    MatsonfordStation,
    #[strum(serialize = "County Line Station - NHSL")]
    CountyLineStation,
    #[strum(serialize = "Radnor Station - NHSL")]
    RadnorStation,
    #[strum(serialize = "Villanova Station - NHSL")]
    VillanovaStation,
    #[strum(serialize = "Stadium Station - NHSL")]
    StadiumStation,
    #[strum(serialize = "Garrett Hill Station - NHSL")]
    GarrettHillStation,
    #[strum(serialize = "Roberts Rd Station - NHSL")]
    RobertsRdStation,
    #[strum(serialize = "Bryn Mawr Station - NHSL")]
    BrynMawrStation,
    #[strum(serialize = "Haverford Station - NHSL")]
    HaverfordStation,
    #[strum(serialize = "Ardmore Junction Station - NHSL")]
    ArdmoreJunctionStation,
    #[strum(serialize = "Wynnewood Rd Station - NHSL")]
    WynnewoodRdStation,
    #[strum(serialize = "Beechwood Brookline Station - NHSL")]
    BeechwoodBrooklineStation,
    #[strum(serialize = "Penfield Station Manoa Rd - NHSL")]
    PenfieldStationManoaRd,
    #[strum(serialize = "Township Line Rd Station - NHSL")]
    TownshipLineRdStation,
    #[strum(serialize = "Parkview Station - NHSL")]
    ParkviewStation,
    #[strum(serialize = "69th St Transportation Center - NHSL")]
    SixtyNinthStTransportationCenter,
}
