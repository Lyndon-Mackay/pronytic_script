use std::fmt;

use lalrpop_util::lalrpop_mod;
use logos::Logos;
use rust_decimal::prelude::*;

use crate::{LexicalError, common::DataParser};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum ShipToken {
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u8>().expect("parsing u8"), priority = 5)]
    Number(u8),

    #[regex(r"(\d+\.\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[token("=")]
    Equal,

    #[token("name")]
    Name,
    #[token("asset_location")]
    AssetLocation,
    #[token("ship_class")]
    ShipClass,

    #[token("survey")]
    Survey,

    #[token("scale")]
    Scale,

    #[token("starts_with")]
    StartsWith,
}

impl fmt::Display for ShipToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Clone, Default, Debug)]
pub enum ShipClass {
    #[default]
    Survey,
}

lalrpop_mod!(pub ship);
#[derive(Clone, Default, Debug)]
pub struct ShipData {
    pub id: String,
    pub name: String,

    pub asset_location: String,

    pub ship_class: ShipClass,

    pub scale: f32,

    pub starts_with: bool,
}

pub enum Field {
    Name(String),
    AssetLocation(String),
    ShipClass(ShipClass),
    Scale(f32),
    StartsWith(bool),
}

impl<'s> DataParser<'s> for ShipData {
    type Token = ShipToken;

    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<Self>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        ship::ShipDataParser::new().parse(tokens)
    }
}
