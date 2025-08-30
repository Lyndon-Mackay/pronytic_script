use std::fmt;

use lalrpop_util::lalrpop_mod;
use logos::Logos;
use rust_decimal::prelude::*;

use crate::{
    LexicalError,
    common::{DataParser, GoodConsumes},
};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum StapledonToken {
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

    #[token("swarm_asset")]
    SwarmAsset,
    #[token("receiver_asset")]
    ReceiverAsset,

    #[token("consumes")]
    Consumes,
    #[token("upkeep")]
    Upkeep,
    #[token("good_id")]
    GoodId,
    #[token("amount")]
    Amount,

    #[token("power")]
    Power,

    #[token("time")]
    Time,
}

impl fmt::Display for StapledonToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub stapledon_swarm);

#[derive(Clone, Default, Debug)]
pub struct StapledonSwarmData {
    pub level: u8,
    pub name: String,

    pub swarm_asset: String,
    pub receiver_asset: String,

    pub power: Decimal,
    pub time: u8,

    pub costs: Vec<GoodConsumes>,
    pub upkeep: Vec<GoodConsumes>,
}

pub enum Field {
    Name(String),
    SwarmAsset(String),
    ReceiverAsset(String),
    Cost(Vec<GoodConsumes>),
    Upkeep(Vec<GoodConsumes>),
    Time(u8),
    Power(Decimal),
}

impl<'s> DataParser<'s, StapledonToken, StapledonSwarmData> for StapledonSwarmData {
    fn parse_tokens(
        tokens: Vec<(usize, StapledonToken, usize)>,
    ) -> Result<Vec<StapledonSwarmData>, lalrpop_util::ParseError<usize, StapledonToken, String>>
    {
        stapledon_swarm::StapledonDataParser::new().parse(tokens)
    }
}
