use crate::errors::Error;
use serde::Deserialize;
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
    CenterCity,
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
            Self::CenterCity => "CC",
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
pub enum RegionalRailStop {
    // Airport Line Stops
    #[strum(
        serialize = "Airport Terminal E F",
        serialize = "Airport Terminal E-F",
        to_string = "Airport Terminal E F"
    )]
    AirportTerminalEF,
    #[strum(
        serialize = "Airport Terminal C D",
        serialize = "Airport Terminal C-D",
        to_string = "Airport Terminal C D"
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
    NorthPhiladelphia,
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
    #[strum(serialize = "Elwyn", serialize = "Elwyn Station", to_string = "Elwyn")]
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
    #[strum(serialize = "Fernwood-Yeadon", serialize = "Fernwood")]
    FernwoodYeadon,
    Angora,
    #[strum(serialize = "49th Street")]
    FortyNinthStreet,

    // Manayunk/Norristown Line Stops
    #[strum(
        serialize = "Norristown - Elm Street",
        serialize = "Norristown Elm Street",
        to_string = "Norristown - Elm Street"
    )]
    NorristownElmStreet,
    MainStreet,
    #[strum(
        serialize = "Norristown T.C.",
        serialize = "Norristown",
        serialize = "Norristown TC"
    )]
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
        serialize = "30th Street Gray",
        serialize = "Gray 30th St",
        to_string = "Gray 30th Street"
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
            Self::NorristownTC => Ok(90226),
            Self::MainStreet => Ok(90227),
            Self::NorristownElmStreet => Ok(90228),
            Self::Wawa => Ok(90300),
            Self::Elwyn => Ok(90301),
            Self::Media => Ok(90302),
            Self::MoylanRoseValley => Ok(90303),
            Self::Wallingford => Ok(90304),
            Self::Swarthmore => Ok(90305),
            Self::Morton => Ok(90306),
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
            Self::Neshaminy => Ok(90323),
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
            Self::DelawareValleyCollege => Ok(90537),
            Self::Doylestown => Ok(90538),
            Self::NinthStreetLansdale => Ok(90539),
            Self::Trenton => Ok(90701),
            Self::Levittown => Ok(90702),
            Self::Bristol => Ok(90703),
            Self::Croydon => Ok(90704),
            Self::Eddington => Ok(90705),
            Self::CornwellsHeights => Ok(90706),
            Self::Torresdale => Ok(90707),
            Self::HolmesburgJct => Ok(90708),
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
            Self::NorthPhiladelphia => Ok(90810),
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
            Self::Cynwyd => Ok((40.0066667, -75.2316667)),
            Self::Bala => Ok((40.0011111, -75.2277778)),
            Self::WynnefieldAvenue => Ok((39.9900000, -75.2255556)),
            Self::Gray30thStreet => Ok((39.9566667, -75.1816667)),
            Self::SuburbanStation => Ok((39.9538889, -75.1677778)),
            Self::JeffersonStation => Ok((39.9525000, -75.1580556)),
            Self::TempleUniversity => Ok((39.9813889, -75.1494444)),
            Self::NorthBroad => Ok((39.9922222, -75.1538889)),
            Self::WayneJunction => Ok((40.0222222, -75.1600000)),
            Self::Newark => Ok((39.6705556, -75.7527778)),
            Self::ChurchmansCrossing => Ok((39.6950000, -75.6725000)),
            Self::Wilmington => Ok((39.7372222, -75.5511111)),
            Self::Claymont => Ok((39.7977778, -75.4522222)),
            Self::MarcusHook => Ok((39.8216667, -75.4194444)),
            Self::HighlandAvenue => Ok((39.8336111, -75.3933333)),
            Self::Chester => Ok((39.8497222, -75.3600000)),
            Self::Eddystone => Ok((39.8572222, -75.3422222)),
            Self::CrumLynne => Ok((39.8719444, -75.3311111)),
            Self::RidleyPark => Ok((39.8805556, -75.3222222)),
            Self::ProspectParkMoore => Ok((39.8883333, -75.3088889)),
            Self::Norwood => Ok((39.8916667, -75.3016667)),
            Self::Glenolden => Ok((39.8963889, -75.2900000)),
            Self::Folcroft => Ok((39.9005556, -75.2797222)),
            Self::SharonHill => Ok((39.9044444, -75.2708333)),
            Self::CurtisPark => Ok((39.9080556, -75.2650000)),
            Self::Darby => Ok((39.9130556, -75.2544444)),
            Self::Allegheny => Ok((40.0036111, -75.1647222)),
            Self::EastFalls => Ok((40.0113889, -75.1919444)),
            Self::Wissahickon => Ok((40.0166667, -75.2102778)),
            Self::Manayunk => Ok((40.0269444, -75.2250000)),
            Self::IvyRidge => Ok((40.0341667, -75.2355556)),
            Self::Miquon => Ok((40.0586111, -75.2663889)),
            Self::SpringMill => Ok((40.0741667, -75.2861111)),
            Self::Conshohocken => Ok((40.0722222, -75.3086111)),
            Self::NorristownTC => Ok((40.1127778, -75.3441667)),
            Self::MainStreet => Ok((40.1172222, -75.3486111)),
            Self::NorristownElmStreet => Ok((40.1208333, -75.3450000)),
            Self::Wawa => Ok((39.901147, -75.459633)),
            Self::Elwyn => Ok((39.9075000, -75.4116667)),
            Self::Media => Ok((39.9144444, -75.3950000)),
            Self::MoylanRoseValley => Ok((39.9061111, -75.3886111)),
            Self::Wallingford => Ok((39.9036111, -75.3719444)),
            Self::Swarthmore => Ok((39.9022222, -75.3508333)),
            Self::Morton => Ok((39.9077778, -75.3288889)),
            Self::Secane => Ok((39.9158333, -75.3097222)),
            Self::Primos => Ok((39.9216667, -75.2983333)),
            Self::CliftonAldan => Ok((39.9266667, -75.2902778)),
            Self::Gladstone => Ok((39.9327778, -75.2822222)),
            Self::Lansdowne => Ok((39.9375000, -75.2708333)),
            Self::FernwoodYeadon => Ok((39.9397222, -75.2558333)),
            Self::Angora => Ok((39.9447222, -75.2386111)),
            Self::FortyNinthStreet => Ok((39.9436111, -75.2166667)),
            Self::Noble => Ok((40.1044444, -75.1241667)),
            Self::Rydal => Ok((40.1075000, -75.1105556)),
            Self::Meadowbrook => Ok((40.1113889, -75.0925000)),
            Self::Bethayres => Ok((40.1166667, -75.0683333)),
            Self::Philmont => Ok((40.1219444, -75.0436111)),
            Self::ForestHills => Ok((40.1277778, -75.0205556)),
            Self::Somerton => Ok((40.1305556, -75.0119444)),
            Self::Trevose => Ok((40.1402778, -74.9825000)),
            Self::Neshaminy => Ok((40.1469444, -74.9616667)),
            Self::Langhorne => Ok((40.1608333, -74.9125000)),
            Self::Woodbourne => Ok((40.1925000, -74.8891667)),
            Self::Yardley => Ok((40.2352778, -74.8305556)),
            Self::WestTrenton => Ok((40.2577778, -74.8152778)),
            Self::AirportTerminalEF => Ok((39.8794444, -75.2397222)),
            Self::AirportTerminalCD => Ok((39.8780556, -75.2400000)),
            Self::AirportTerminalB => Ok((39.8772222, -75.2413889)),
            Self::AirportTerminalA => Ok((39.8761111, -75.2452778)),
            Self::Eastwick => Ok((39.8927778, -75.2438889)),
            Self::PennMedicineStation => Ok((39.9480556, -75.1902778)),
            Self::FernRockTC => Ok((40.0405556, -75.1347222)),
            Self::MelrosePark => Ok((40.0594444, -75.1291667)),
            Self::ElkinsPark => Ok((40.0713889, -75.1277778)),
            Self::JenkintownWyncote => Ok((40.0927778, -75.1375000)),
            Self::Glenside => Ok((40.1013889, -75.1536111)),
            Self::Ardsley => Ok((40.1141667, -75.1530556)),
            Self::Roslyn => Ok((40.1208333, -75.1341667)),
            Self::Crestmont => Ok((40.1333333, -75.1186111)),
            Self::WillowGrove => Ok((40.1438889, -75.1141667)),
            Self::Hatboro => Ok((40.1761111, -75.1025000)),
            Self::Warminster => Ok((40.1952778, -75.0891667)),
            Self::Thorndale => Ok((39.9927778, -75.7636111)),
            Self::Downingtown => Ok((40.0022222, -75.7102778)),
            Self::Whitford => Ok((40.0147222, -75.6380556)),
            Self::Exton => Ok((40.0191667, -75.6227778)),
            Self::Malvern => Ok((40.0363889, -75.5155556)),
            Self::Paoli => Ok((40.0430556, -75.4827778)),
            Self::Daylesford => Ok((40.0430556, -75.4605556)),
            Self::Berwyn => Ok((40.0480556, -75.4422222)),
            Self::Devon => Ok((40.0472222, -75.4227778)),
            Self::Strafford => Ok((40.0494444, -75.4030556)),
            Self::Wayne => Ok((40.0458333, -75.3866667)),
            Self::StDavids => Ok((40.0438889, -75.3725000)),
            Self::Radnor => Ok((40.0447222, -75.3588889)),
            Self::Villanova => Ok((40.0383333, -75.3416667)),
            Self::Rosemont => Ok((40.0277778, -75.3266667)),
            Self::BrynMawr => Ok((40.0219444, -75.3163889)),
            Self::Haverford => Ok((40.0138889, -75.2997222)),
            Self::Ardmore => Ok((40.0083333, -75.2902778)),
            Self::Wynnewood => Ok((40.0027778, -75.2725000)),
            Self::Narberth => Ok((40.0047222, -75.2613889)),
            Self::Merion => Ok((39.9986111, -75.2513889)),
            Self::Overbrook => Ok((39.9894444, -75.2494444)),
            Self::NorthHills => Ok((40.1119444, -75.1694444)),
            Self::Oreland => Ok((40.1183333, -75.1838889)),
            Self::FortWashington => Ok((40.1358333, -75.2122222)),
            Self::Ambler => Ok((40.1536111, -75.2247222)),
            Self::Penllyn => Ok((40.1700000, -75.2441667)),
            Self::GwyneddValley => Ok((40.1847222, -75.2569444)),
            Self::NorthWales => Ok((40.2141667, -75.2772222)),
            Self::Pennbrook => Ok((40.2302778, -75.2816667)),
            Self::Lansdale => Ok((40.2427778, -75.2850000)),
            Self::Fortuna => Ok((40.2594444, -75.2661111)),
            Self::Colmar => Ok((40.2683333, -75.2544444)),
            Self::LinkBelt => Ok((40.2738889, -75.2466667)),
            Self::Chalfont => Ok((40.2877778, -75.2097222)),
            Self::NewBritain => Ok((40.2975000, -75.1797222)),
            Self::DelawareValleyCollege => Ok((40.2972222, -75.1616667)),
            Self::Doylestown => Ok((40.3063889, -75.1302778)),
            Self::NinthStreetLansdale => Ok((40.2500000, -75.2791667)),
            Self::Trenton => Ok((40.2177778, -74.7550000)),
            Self::Levittown => Ok((40.1402778, -74.8169444)),
            Self::Bristol => Ok((40.1047222, -74.8547222)),
            Self::Croydon => Ok((40.0936111, -74.9066667)),
            Self::Eddington => Ok((40.0830556, -74.9336111)),
            Self::CornwellsHeights => Ok((40.0716667, -74.9522222)),
            Self::Torresdale => Ok((40.0544444, -74.9844444)),
            Self::HolmesburgJct => Ok((40.0327778, -75.0236111)),
            Self::Tacony => Ok((40.0233333, -75.0388889)),
            Self::Bridesburg => Ok((40.0105556, -75.0697222)),
            Self::NorthPhiladelphiaAmtrak => Ok((39.9972222, -75.1550000)),
            Self::Wister => Ok((40.0361111, -75.1611111)),
            Self::Germantown => Ok((40.0375000, -75.1716667)),
            Self::WashingtonLane => Ok((40.0508333, -75.1713889)),
            Self::Stenton => Ok((40.0605556, -75.1786111)),
            Self::Sedgwick => Ok((40.0627778, -75.1852778)),
            Self::MountAiry => Ok((40.0652778, -75.1908333)),
            Self::Wyndmoor => Ok((40.0733333, -75.1966667)),
            Self::Gravers => Ok((40.0775000, -75.2016667)),
            Self::ChestnutHillEast => Ok((40.0811111, -75.2072222)),
            Self::ChestnutHillWest => Ok((40.0763889, -75.2083333)),
            Self::Highland => Ok((40.0705556, -75.2111111)),
            Self::StMartins => Ok((40.0658333, -75.2044444)),
            Self::RichardAllenLane => Ok((40.0575000, -75.1947222)),
            Self::Carpenter => Ok((40.0511111, -75.1913889)),
            Self::Upsal => Ok((40.0425000, -75.1900000)),
            Self::Tulpehocken => Ok((40.0352778, -75.1869444)),
            Self::CheltenAvenue => Ok((40.0300000, -75.1808333)),
            Self::QueenLane => Ok((40.0233333, -75.1780556)),
            Self::NorthPhiladelphia => Ok((39.9977778, -75.1563889)),
            Self::Olney => Ok((40.0333333, -75.1227778)),
            Self::Lawndale => Ok((40.0513889, -75.1030556)),
            Self::Cheltenham => Ok((40.0580556, -75.0927778)),
            Self::Ryers => Ok((40.0641667, -75.0863889)),
            Self::FoxChase => Ok((40.0763889, -75.0833333)),
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
