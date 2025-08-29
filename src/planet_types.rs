use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use miette::NamedSource;
use rust_decimal::Decimal;

use logos::Logos;

use crate::{LexicalError, SyntaxError, lex};

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

    #[regex(r"(-?\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[regex(r"[1-9][0-9]*", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"))]
    Number(Decimal),
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    Name(String),

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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
lalrpop_mod!(pub planet_types);
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
pub struct IfCondition {
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
}

#[derive(Clone, Debug, Default)]
pub struct Branch {
    pub if_conditions: Vec<IfCondition>,
    pub else_actions: Vec<Action>,
}

#[derive(Debug, Clone)]
pub enum Field {
    AssetLocation(String),
    GoodsAbundance(Vec<GoodAbundance>),
    Setup(Branch),
    Terraform(Branch),
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
}

#[derive(Clone, Debug)]
pub enum Action {
    None,
    SetAsset(String),
    SetPlanetType(String),
    Branch(Branch),
    SetStored(String, String),
}

#[derive(Clone, Debug, Default)]
pub struct GoodAbundance {
    pub id: String,
    pub mean: Decimal,
    pub std_dev: Decimal,
}

#[derive(Clone, Debug, Default)]
pub struct PlanetTypeDataList {
    pub list: Vec<PlanetTypeData>,
}
#[derive(Clone, Debug, Default)]
pub struct PlanetTypeData {
    pub name: String,
    pub abundances: Vec<GoodAbundance>,
    pub asset_location: String,
    pub setup_conditions: Vec<Branch>,
    pub terraform_conditions: Vec<Branch>,
}

pub(super) fn parse_planet_types_section(file_name: &str, input: &str) -> Vec<PlanetTypeData> {
    let tokens = lex::<Token>(file_name, input);
    let planet_type_parse = planet_types::PlanetTypeListParser::new().parse(tokens);
    match planet_type_parse {
        Ok(s) => s.list,
        Err(e) => match e {
            lalrpop_util::ParseError::InvalidToken { location } => {
                let problem = SyntaxError {
                    src: NamedSource::new(file_name, input.to_string()),
                    bad_bit: (location).into(),
                    advice: Some("Skill issue".to_string()),
                };

                panic!("{:?}", miette::Error::new(problem));
            }
            lalrpop_util::ParseError::UnrecognizedEof { .. } => todo!(),
            lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                let problem = SyntaxError {
                    src: NamedSource::new(file_name, input.to_string()),
                    bad_bit: (token.0, token.2).into(),
                    advice: Some(format!("Expected {} found {}", expected.join(","), token.1)),
                };
                panic!("{:?}", miette::Error::new(problem));
            }
            lalrpop_util::ParseError::ExtraToken { .. } => todo!(),
            lalrpop_util::ParseError::User { .. } => todo!(),
        },
    }
}
