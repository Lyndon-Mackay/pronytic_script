use std::fmt;

use lalrpop_util::lalrpop_mod;
use logos::Logos;

use crate::{
    LexicalError,
    common::{DataParser, PlanetFilter},
};

use rust_decimal::prelude::*;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum DesignationToken {
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u8>().expect("parsing u8"), priority = 5)]
    Number(u8),

    #[regex(r"(-?\d+\.\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[token("=")]
    Equal,

    #[token("(")]
    LeftBracket,
    #[token(")")]
    RightBracket,

    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,

    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,

    #[token("orbital")]
    Orbital,
    #[token("all_orbitals")]
    AllOrbitals,
    #[token("all_planets")]
    AllPlanets,

    #[token("build_planets")]
    PlanetFilters,

    #[token("name")]
    Name,
    #[token("description")]
    Description,

    #[token("building_limit")]
    BuildingLimit,
    #[token("Unlimited")]
    Unlimited,
    #[token("limited")]
    Limited,

    #[token("population_impact")]
    PopulationImpact,

    #[token("tech_required")]
    TechRequired,

    #[token("growth")]
    Growth,
    #[token("min_population")]
    MinPopulation,

    #[token("housing")]
    Housing,
    #[token("managend")]
    Managed,
    #[token("unmanaged")]
    Unmanaged,

    #[token("private_buildings")]
    PrivateBuildings,

    #[token("none")]
    None,
    #[token("always")]
    Always,
}

impl fmt::Display for DesignationToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub designation);

///Parsed serialisation data to send to the game
#[derive(Clone, Debug, Default)]
pub struct DesignationData {
    pub id: String,

    pub name: String,
    pub description: String,

    pub building_limit: BuildingLimit,
    pub housing: Housing,
    pub population_impact: PopulationImpact,

    pub tech_required: Option<String>,

    pub planet_filters: Vec<PlanetFilter>,

    pub private_buildings: PrivateBuildings,
}

/// Designations can have population limits the idea behind this is have
/// low output planets that can largely be in a finished state to avoid micromanaging
#[derive(Clone, Default, Debug)]
pub enum BuildingLimit {
    #[default]
    Unlimited,
    Limited(u8),
}

/// If housing should be required for this planet,
/// with population limited planets requiring the building of housing
/// doesn't feel as interesting for gameplay reasons.
#[derive(Clone, Default, Debug)]
pub enum Housing {
    #[default]
    Managed,
    Unmanaged,
}

/// Private buildings are a way companies expand their revenue
/// independantly of the player, this is to stop the players plans
/// getting disrupted at the early stages of the buildings
#[derive(Clone, Default, Debug)]
pub enum PrivateBuildings {
    #[default]
    None,
    MinPopulation(u8),
    Always,
}

/// This is a way to simulate urban population decline
/// This is there to stop perputual population growth along
/// with giving you a reason to no hyper develop every planet
#[derive(Clone, Default, Debug)]
pub struct PopulationImpact {
    pub growth: Decimal,
    pub min_population: u8,
}

pub enum Field {
    Name(String),
    Description(String),
    Housing(Housing),
    BuildingLimit(BuildingLimit),
    PopulationImpact(PopulationImpact),
    PlanetFilters(Vec<PlanetFilter>),
    PrivateBuildings(PrivateBuildings),
    TechRequired(String),
}

impl<'s> DataParser<'s> for DesignationData {
    type Token = DesignationToken;

    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<Self>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        designation::DesignationDataParser::new().parse(tokens)
    }
}
