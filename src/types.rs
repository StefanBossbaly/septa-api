use serde_derive::Deserialize;
use strum::{Display, EnumString};

pub enum TransportType {
    Bus,
    RegionalRail,
    Nhsl,
    Subway,
    Trolley,
}
#[derive(Debug, Deserialize, Display, EnumString, PartialEq)]
#[strum(serialize_all = "UPPERCASE")]
pub enum ServiceType {
    Express,
    Local,

    #[strum(default)]
    Unknown(String),
}

#[derive(Debug, Deserialize, Display, EnumString, PartialEq)]
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
        }
    }

    pub fn stops(&self) -> Vec<RegionalRailStop> {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Display, EnumString)]
#[strum(serialize_all = "title_case", ascii_case_insensitive)]
pub enum RegionalRailStop {
    // Airport Line Stops
    #[strum(serialize = "Airport Terminal E F")]
    AirportTerminalEF,
    #[strum(serialize = "Airport Terminal C D")]
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
    #[strum(serialize = "Chestnut H East")]
    ChestnutHillEast,
    WashingtonLane,
    MountAiry,

    // Chestnut Hill West Line Stops
    NorthPhiladelphia,
    Upsal,
    #[strum(serialize = "St. Martins")]
    StMartins,
    #[strum(serialize = "Chestnut H West")]
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
    DelawareValleyCollege,
    NewBritain,
    Chalfont,
    LinkBelt,
    Colmar,
    Fortuna,
    #[strum(serialize = "9th Street Lansdale")]
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
    Elwyn,
    Media,
    #[strum(serialize = "Moylan-Rose Valley")]
    MoylanRoseValley,
    Wallingford,
    Swarthmore,
    Morton,
    Secane,
    Primos,
    #[strum(serialize = "Clifton-Aldan")]
    CliftonAldan,
    Gladstone,
    Lansdowne,
    #[strum(serialize = "Fernwood-Yeadon")]
    FernwoodYeadon,
    Angora,
    #[strum(serialize = "49th Street")]
    FortyNinthStreet,

    // Manayunk/Norristown Line Stops
    NorristownElmStreet,
    MainStreet,
    #[strum(serialize = "Norristown T.C.", serialize = "Norristown")]
    NorristownTC,
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
    Exton,
    Malvern,
    Paoli,
    Wayne,
    #[strum(serialize = "St. Davids")]
    StDavids,
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

    // Trenton Line Stops
    Trenton,
    Levittown,
    Bristol,
    Croydon,
    Eddington,
    CornwellsHeights,
    Torresdale,
    HolmesburgJct,
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
    Newark,
    #[strum(serialize = "Churchman's Crossing")]
    ChurchmansCrossing,
    Wilmington,
    Claymont,
    MarcusHook,
    HighlandAvenue,
    Chester,
    Eddystone,
    CrumLynne,
    RidleyPark,
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
    Neshaminy,
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
    JenkintownWyncote,
    #[strum(serialize = "Fern Rock T C")]
    FernRockTC,
    ElkinsPark,
    MelrosePark,

    // Shared Center City Stops
    #[strum(
        serialize = "30th Street Station",
        serialize = "30th St",
        serialize = "Gray 30th Street"
    )]
    Gray30thStreet,
    SuburbanStation,
    JeffersonStation,
    #[strum(serialize = "Temple U", serialize = "Temple University")]
    TempleUniversity,
    PennMedicineStation,

    // Unknown Stop
    #[strum(default)]
    Unknown(String),
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
