use std::fmt;

use lalrpop_util::lalrpop_mod;
use logos::Logos;
use miette::NamedSource;
use rust_decimal::prelude::*;

use crate::{LexicalError, SyntaxError, common::GoodConsumes, lex};

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

impl fmt::Display for Token {
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

pub(super) fn parse_stapledon(file_name: &str, input: &str) -> Vec<StapledonSwarmData> {
    let tokens = lex::<Token>(file_name, input);
    let orbital_parse = stapledon_swarm::StapledonDataParser::new().parse(tokens);
    match orbital_parse {
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
