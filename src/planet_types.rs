use std::{fmt, num::ParseIntError, str::FromStr};

use lalrpop_util::lalrpop_mod;
use miette::{Diagnostic, Error, NamedSource, SourceSpan};
use regex::Regex;
use rust_decimal::Decimal;

use thiserror::Error;
use tracing::*;

use logos::{self, Logos, Source};

use crate::{LexicalError, SyntaxError, handle_lexical_errors};

const SKIP_TEXTS: [&str; 2] = [r"//[^\n\r]*", r"[\s\t\f]+"];

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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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

fn lex(file_name: &str, input: &str) -> Vec<(usize, Token, usize)> {
    let mut lex = Token::lexer(input);
    let mut tokens = Vec::new();
    while let Some(tok) = lex.next() {
        let token = match tok {
            Ok(t) => t,
            Err(e) => match e {
                LexicalError::InvalidInteger(parse_int_error) => todo!(),
                LexicalError::InvalidToken => {
                    let last: usize = tokens.last().map(|(_, _, x)| *x).unwrap_or_default();

                    handle_lexical_errors(file_name, e, input, last);
                }
            },
        };
        let span = lex.span();
        tokens.push((span.start, token, span.end));
    }
    tokens
}

pub(super) fn parse_planet_types_section(file_name: &str, input: &str) -> Vec<PlanetTypeData> {
    let tokens = lex(file_name, input);
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
            lalrpop_util::ParseError::UnrecognizedEof { location, expected } => todo!(),
            lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                let problem = SyntaxError {
                    src: NamedSource::new(file_name, input.to_string()),
                    bad_bit: (token.0, token.2).into(),
                    advice: Some(format!("Expected {} found {}", expected.join(","), token.1)),
                };
                panic!("{:?}", miette::Error::new(problem));
            }
            lalrpop_util::ParseError::ExtraToken { token } => todo!(),
            lalrpop_util::ParseError::User { error } => todo!(),
        },
    }
}
