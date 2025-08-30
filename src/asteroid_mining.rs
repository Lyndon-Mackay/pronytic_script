use std::fmt;

use rust_decimal::prelude::*;

use lalrpop_util::{ParseError, lalrpop_mod};
use logos::{self, Logos};

use crate::{
    LexicalError,
    common::{DataParser, GoodConsumes},
};

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

impl<'s> DataParser<'s, Token, AsteroidMiningData> for AsteroidMiningData {
    fn parse_tokens(
        tokens: Vec<(usize, Token, usize)>,
    ) -> Result<Vec<AsteroidMiningData>, ParseError<usize, Token, String>> {
        asteroid_mining::AsteroidMiningDataParser::new().parse(tokens)
    }
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
