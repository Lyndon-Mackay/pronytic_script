use std::fmt;

use rust_decimal::prelude::*;

use lalrpop_util::lalrpop_mod;
use logos::{self, Logos};

use crate::{LexicalError, SyntaxError, common::GoodConsumes, lex};
use miette::NamedSource;

//TODO! this number tokenising is inconsistent with other token types I should change the others to split decimal numbers as consistently
#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum Token {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u8>().expect("parsing u8"), priority = 5)]
    Number(u8),

    #[regex(r"(-?\d+\.\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[token("=")]
    Equal,

    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,

    #[token(":")]
    Colon,

    #[token("name")]
    Name,

    #[token("depot_asset")]
    DepotAsset,
    #[token("ship_asset")]
    ShipAsset,

    #[token("consumes")]
    Consumes,
    #[token("produces")]
    Produces,
    #[token("good_id")]
    GoodId,
    #[token("amount")]
    Amount,

    #[token("power")]
    Power,

    #[token("time")]
    Time,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
lalrpop_mod!(pub asteroid_mining);

#[derive(Clone, Default, Debug)]
pub struct AsteroidMiningData {
    pub level: u8,
    pub name: String,

    pub depot_asset: String,
    pub ship_asset: String,

    pub costs: Vec<GoodConsumes>,
    pub produces: Vec<GoodConsumes>,

    pub power: Decimal,
    pub time: u8,
}

pub enum Field {
    Name(String),
    DepotAsset(String),
    ShipAsset(String),
    Consumes(Vec<GoodConsumes>),
    Produces(Vec<GoodConsumes>),
    Time(u8),
    Power(Decimal),
}

pub(super) fn parse_asteroid_mining(file_name: &str, input: &str) -> Vec<AsteroidMiningData> {
    let tokens = lex::<Token>(file_name, input);
    let shipyard_parse = asteroid_mining::AsteroidMiningDataParser::new().parse(tokens);
    match shipyard_parse {
        Ok(list) => list,
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
