use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use logos::{self, Logos};
use rust_decimal::prelude::*;

use crate::{
    LexicalError,
    common::{DataParser, GoodConsumes},
};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum OrbitalToken {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u8>().expect("parsing u8"), priority = 5)]
    Number(u8),

    #[regex(r"(\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
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
    #[token("building_limit")]
    BuildingLimit,
}

impl fmt::Display for OrbitalToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub orbital);
#[derive(Clone, Default, Debug)]
pub struct OrbitalData {
    pub level: u8,
    pub name: String,
    pub asset_location: String,

    pub costs: Vec<GoodConsumes>,

    pub time: u8,
    pub building_limit: u8,
}

pub enum Field {
    Name(String),
    AssetLocation(String),
    Consumes(Vec<GoodConsumes>),
    Time(u8),
    BuildingLimit(u8),
}

impl<'s> DataParser<'s> for OrbitalData {
    type Token = OrbitalToken;
    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<OrbitalData>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        orbital::OrbitalDataParser::new().parse(tokens)
    }
}
