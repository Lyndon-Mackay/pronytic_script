use std::{fmt, str::FromStr};

use rust_decimal::prelude::*;

use lalrpop_util::lalrpop_mod;

use crate::{LexicalError, common::DataParser};
use logos::{self, Logos};

#[derive(Clone, Debug, Default)]
pub struct CustomGood {
    pub id: String,
    pub amount: Decimal,
}

#[derive(Clone, Debug, Default)]
pub struct AtmosphereImpact {
    pub added_equilibrium: Decimal,
    pub rate: Decimal,
}

#[derive(Clone, Debug, Default)]
pub struct MagnetosphereImpact {
    pub added_equilibrium: Decimal,
    pub rate: Decimal,
}

#[derive(Clone, Debug, Default)]
pub struct Station {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub scale: f32,

    pub path: String,
    //TODO animation information
}
/// Building data to send to game
/// this is only made for serialisation
/// actual data structure in game is different
#[derive(Clone, Debug)]
pub struct BuildingData {
    pub id: String,
    pub name: String,

    pub planet_filters: Vec<String>,

    pub initial: bool,
    pub unique: bool,

    pub energy: Decimal,

    pub costs: Vec<CustomGood>,
    pub private_costs: Decimal,
    pub consumes: Vec<CustomGood>,
    pub produces: Vec<CustomGood>,

    pub housing: u64,
    pub workers: u64,

    pub private_sector: bool,

    pub stations: Vec<Station>,

    pub magnetosphere_equilibrium: MagnetosphereImpact,
    pub atmosphere_equilibrium: AtmosphereImpact,

    pub temperature_change: Decimal,
    pub water_change: Decimal,
    pub breathable_change: Decimal,

    pub tech_needed: Option<String>,
    pub upgrades_from: Option<String>,

    pub prosperity_per_job: Decimal,
}

impl Default for BuildingData {
    fn default() -> Self {
        BuildingData {
            id: "".to_string(),
            name: "".to_string(),
            planet_filters: Vec::new(),
            initial: false,
            unique: false,
            energy: Decimal::ZERO,

            costs: Vec::new(),
            private_costs: Decimal::ZERO,
            consumes: Vec::new(),
            produces: Vec::new(),

            housing: 0,
            workers: 0,

            private_sector: false,

            stations: vec![],

            magnetosphere_equilibrium: MagnetosphereImpact::default(),
            atmosphere_equilibrium: AtmosphereImpact::default(),
            temperature_change: Decimal::ZERO,
            water_change: Decimal::ZERO,
            breathable_change: Decimal::ZERO,
            tech_needed: None,
            upgrades_from: None,
            prosperity_per_job: Decimal::ONE,
        }
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum Token {
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("=")]
    Equal,

    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,

    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,

    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),
    #[regex(r"(-\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    NegativeNumber(Decimal),

    #[token("id")]
    Id,
    #[token("name")]
    Name,

    #[token("build_planets")]
    PlanetFilters,

    #[token("initial")]
    Initial,
    #[token("unique")]
    Unique,

    #[token("energy")]
    Energy,

    #[token("private_cost")]
    PrivateCosts,

    #[token("stations")]
    Stations,

    #[token("costs")]
    Costs,
    #[token("consumes")]
    Consumes,
    #[token("produces")]
    Produces,

    #[token("housing")]
    Housing,
    #[token("workers")]
    Workers,

    #[token("private_sector")]
    PrivateSector,

    #[token("magnetosphere_equilibrium")]
    MagnetosphereEquilibrium,

    #[token("atmosphere_equilibrium")]
    AtmosphereEquilibrium,

    #[token("temperature_change")]
    TemperatureChange,

    #[token("water_change")]
    WaterChange,

    #[token("breathable_change")]
    BreathableChange,

    #[token("tech_needed")]
    TechNeeded,
    #[token("upgrades_from")]
    UpgradesFrom,

    #[token("prosperity_per_job")]
    ProsperityPerJob,

    #[token("rate")]
    Rate,
    #[token("added")]
    Added,

    #[token("x")]
    X,
    #[token("y")]
    Y,
    #[token("z")]
    Z,
    #[token("scale")]
    Scale,
    #[token("path")]
    Path,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub buildings);
pub enum Field {
    Name(String),
    PlanetFilters(Vec<String>),
    Initial(bool),
    Unique(bool),
    Energy(Decimal),
    PrivateCosts(Decimal),
    Costs(Vec<CustomGood>),
    Consumes(Vec<CustomGood>),
    Produces(Vec<CustomGood>),
    Housing(u64),
    Workers(u64),
    PrivateSector(bool),
    Magnetosphere(MagnetosphereImpact),
    Atmosphere(AtmosphereImpact),
    TemperatureChange(Decimal),
    WaterChange(Decimal),
    BreathableChange(Decimal),
    TechNeeded(String),
    UpgradesFrom(String),
    ProsperityPerJob(Decimal),
    Stations(Vec<Station>),
}

pub enum StationField {
    X(f32),
    Y(f32),
    Z(f32),
    Scale(f32),
    Path(String),
}

impl<'s> DataParser<'s, Token, BuildingData> for BuildingData {
    fn parse_tokens(
        tokens: Vec<(usize, Token, usize)>,
    ) -> Result<Vec<BuildingData>, lalrpop_util::ParseError<usize, Token, String>> {
        buildings::BuildingsParser::new().parse(tokens)
    }
}
