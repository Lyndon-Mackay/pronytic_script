use std::fmt;

use crate::{LexicalError, common::DataParser};
use lalrpop_util::lalrpop_mod;
use logos::{self, Logos};
use rust_decimal::prelude::*;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum RankToken {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u16>().expect("parsing u8"), priority = 5)]
    Number(u16),

    #[regex(r"(-?\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[token("=")]
    Equal,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,

    #[token("number_of_stars")]
    NumStars,

    #[token("stockpile_max")]
    StockpileMax,
    #[token("huck_max")]
    HuckMax,
    #[token("level")]
    Level,
    #[token("name")]
    Name,
    #[token("description")]
    Description,
}

impl fmt::Display for RankToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub ranks);

#[derive(Clone, Default, Debug)]
pub struct RankData {
    pub level: u16,
    pub name: String,
    pub number_of_stars: u16,
    pub stockpile_max: u16,
    pub huck_max: Decimal,
    pub description: Option<String>,
}
impl<'s> DataParser<'s> for RankData {
    type Token = RankToken;

    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<Self>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        ranks::RankDataParser::new().parse(tokens)
    }
}

pub enum Field {
    Name(String),
    NumStars(u16),
    StockpileMax(u16),
    HuckMax(Decimal),
    Description(String),
}
