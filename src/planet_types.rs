use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use rust_decimal::Decimal;

use logos::Logos;

use crate::{
    LexicalError,
    common::{DataParser, GoodAbundance},
};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum PlanetTypeToken {
    #[token("true")]
    True,
    #[token("false")]
    False,

    #[token("=")]
    Equal,

    #[regex(r"(-?\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[regex(r"[1-9][0-9]*", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"))]
    Number(Decimal),
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    Name(String),

    #[token("class")]
    Class,

    #[token("set_asset")]
    SetAsset,
    #[token("set_planet_type")]
    SetPlanetType,

    #[token("goods_abundance")]
    GoodsAbundance,

    #[token("mean")]
    Mean,
    #[token("std_dev")]
    StdDev,

    #[token(":")]
    Colon,

    #[token("(")]
    RightBracket,
    #[token(")")]
    LeftBracket,

    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,

    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,

    //section
    #[token("setup")]
    Setup,
    #[token("on_terraform")]
    OnTerraform,

    //Conditionals
    #[token("if")]
    If,
    #[token("else")]
    Else,

    //comparitors
    #[token("EQ")]
    Eq,
    #[token("NE")]
    Ne,
    #[token("GT")]
    Gt,
    #[token("GE")]
    Ge,
    #[token("LT")]
    Lt,
    #[token("LE")]
    Le,

    #[token("IN")]
    In,

    #[token("&")]
    Ampersand,

    #[token("star_type")]
    StarType,
    #[token("oxygen_level")]
    OxygenLevel,
    #[token("temperature_celsius")]
    TemperatureCelsius,
    #[token("temperature_kelvin")]
    TemperatureKelvin,
    #[token("water_level")]
    WaterLevel,
    #[token("magnetosphere")]
    Magnetosphere,
    #[token("atmosphere")]
    Atmosphere,
    #[token("goods_base")]
    GoodsBase,

    #[token("stored")]
    Stored,
    #[token("stored_number")]
    StoredNumber,

    #[token("rand_of_string")]
    RandomOfString,
}

impl fmt::Display for PlanetTypeToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
lalrpop_mod!(pub planet_types);

#[derive(Debug, Clone)]
pub enum Field {
    AssetLocation(String),
    GoodsAbundance(Vec<GoodAbundance>),
    Setup(Branch),
    Terraform(Vec<Branch>),
}

#[derive(Clone, Debug, Default)]
pub struct PlanetTypeData {
    pub name: String,
    pub planet_class: PlanetClass,
    pub abundances: Vec<GoodAbundance>,
    pub asset_location: String,
    pub setup_conditions: Vec<Branch>,
    pub terraform_conditions: Vec<Branch>,
}

impl<'s> DataParser<'s> for PlanetTypeData {
    type Token = PlanetTypeToken;
    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<PlanetTypeData>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        planet_types::PlanetTypeListParser::new().parse(tokens)
    }
}

/// The group the planet type falls under
#[derive(Clone, Debug, Default)]
pub enum PlanetClass {
    #[default]
    Rocky,
    Atmospheric,
    Gas,
}

#[derive(Clone, Debug, Default)]
pub struct Branch {
    pub if_conditions: Vec<IfCondition>,
    pub else_actions: Vec<Action>,
}
#[derive(Clone, Debug)]
pub struct IfCondition {
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
}

#[derive(Clone, Debug)]
pub enum Condition {
    Eq(Value, Value),
    Gt(Value, Value),
    Ge(Value, Value),
    Lt(Value, Value),
    Le(Value, Value),
    Ne(Value, Value),
}

#[derive(Clone, Debug)]
pub enum Action {
    None,
    SetAsset(String),
    SetPlanetType(String),
    Branch(Branch),
    SetStored(String, String),
    SetStoredRandom(String, Vec<String>),
}

#[derive(Clone, Debug)]
pub enum Value {
    Decimal(Decimal),
    OxygenLevel,
    TemperatureCelsius,
    TemperatureKelvin,
    WaterLevel,
    Magnetosphere,
    Atmosphere,

    GoodsAbundance(String),

    StarType,
    String(String),
    StringLookup(String),
}
