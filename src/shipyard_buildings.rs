use std::fmt;

use logos::{self, Logos};

use crate::{LexicalError, building::CustomGood, common::DataParser};

use lalrpop_util::lalrpop_mod;
use rust_decimal::prelude::*;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum ShipyardBuildingToken {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u8>().expect("parsing u8"), priority = 5)]
    Number(u8),

    #[regex(r"(-?\d+\.\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[token("=")]
    Equal,
    #[token(":")]
    Colon,

    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,

    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,

    #[token("name")]
    Name,

    #[token("costs")]
    Costs,

    #[token("good_id")]
    GoodId,

    #[token("amount")]
    Amount,

    #[token("level_required")]
    LevelRequired,

    #[token("base_strength")]
    BaseStrength,

    #[token("fleet_strength")]
    FleetStrength,

    #[token("upkeep")]
    Upkeep,

    #[token("power")]
    Power,

    #[token("time")]
    Time,
}
impl fmt::Display for ShipyardBuildingToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub shipyard_buildings);

#[derive(Clone, Debug, Default)]
pub struct ShipyardBuildingData {
    pub id: String,
    pub name: String,

    /// Shipyards level needed to build this
    pub level_required: u8,

    /// How much the building adds to the base's strength
    pub base_strength: Decimal,
    /// How much the building adds to the fleet's strength
    pub fleet_strength: Decimal,

    pub costs: Vec<CustomGood>,
    /// How much huck each building costs to maintain
    pub upkeep: Decimal,

    /// How much power this stations costs to use
    pub power: Decimal,

    pub time: u8,
}

pub enum Field {
    Name(String),
    LevelRequired(u8),
    Costs(Vec<CustomGood>),
    Time(u8),
    Upkeep(Decimal),
    Power(Decimal),
    BaseStrength(Decimal),
    FleetStrength(Decimal),
}

impl<'s> DataParser<'s> for ShipyardBuildingData {
    type Token = ShipyardBuildingToken;

    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<Self>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        shipyard_buildings::ShipyardBuildingDataParser::new().parse(tokens)
    }
}
