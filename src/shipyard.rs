use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use logos::{self, Logos};
use rust_decimal::prelude::*;

use miette::NamedSource;

use crate::{LexicalError, SyntaxError, common::GoodConsumes, lex};

//TODO! this number tokenising is inconsistent with other token types I should change the others to split decimal numbers as consistently
#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum Token {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u8>().expect("parsing u8"), priority = 5)]
    Number(u8),

    #[regex(r"(\d+\.\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
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
    #[token("asset_location")]
    AssetLocation,

    #[token("consumes")]
    Consumes,
    #[token("good_id")]
    GoodId,
    #[token("amount")]
    Amount,

    #[token("time")]
    Time,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub shipyard);
#[derive(Clone, Default, Debug)]
pub struct ShipyardData {
    pub level: u8,
    pub name: String,
    pub asset_location: String,

    pub costs: Vec<GoodConsumes>,
    pub time: u8,
}

pub enum Field {
    Name(String),
    AssetLocation(String),
    Consumes(Vec<GoodConsumes>),
    Time(u8),
}

pub(super) fn parse_shipyard(file_name: &str, input: &str) -> Vec<ShipyardData> {
    let tokens = lex::<Token>(file_name, input);
    let shipyard_parse = shipyard::ShipyardDataParser::new().parse(tokens);
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
