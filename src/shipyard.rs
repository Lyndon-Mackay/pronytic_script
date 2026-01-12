use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use logos::{self, Logos};
use rust_decimal::prelude::*;

use crate::{
    LexicalError,
    common::{DataParser, GoodConsumes},
};

//TODO! this number tokenising is inconsistent with other token types I should change the others to split decimal numbers as consistently
#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum ShipyardToken {
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

    #[token("star_class")]
    StarClass,

    #[token("armaments")]
    Armaments,

    #[token("base_strength")]
    BaseStrength,
    #[token("fleet_strength")]
    FleetStrength,
}

impl fmt::Display for ShipyardToken {
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

    /// Gives ships a bonus when fighting in same star system
    /// It was built in
    pub star_class: bool,

    pub armaments: bool,

    pub base_strength: Decimal,
    pub fleet_strength: Decimal,
}

pub enum Field {
    Name(String),
    AssetLocation(String),
    Consumes(Vec<GoodConsumes>),
    Time(u8),
    BaseStrength(Decimal),
    FleetStrength(Decimal),
    StarClass(bool),
    Armaments(bool),
}
impl<'s> DataParser<'s> for ShipyardData {
    type Token = ShipyardToken;
    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<ShipyardData>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        shipyard::ShipyardDataParser::new().parse(tokens)
    }
}
