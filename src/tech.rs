use std::fmt;

use lalrpop_util::lalrpop_mod;

use logos::{self, Logos};

use crate::{LexicalError, common::DataParser};

#[derive(Clone, Debug, Default)]
pub struct TechData {
    pub id: String,
    pub name: String,
    pub time: u8,
    pub description: String,
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum TechToken {
    #[token("=")]
    Equal,
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),
    #[regex(r#"\d+"#, |lex| lex.slice().parse::<u8>().unwrap())]
    Number(u8),
    #[token("name")]
    Name,
    #[token("time")]
    Time,
    #[token("description")]
    Description,
}

impl fmt::Display for TechToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
pub enum Field {
    Time(u8),
    Name(String),
    Description(String),
}

lalrpop_mod!(pub tech);

impl<'s> DataParser<'s, TechToken, TechData> for TechData {
    fn parse_tokens(
        tokens: Vec<(usize, TechToken, usize)>,
    ) -> Result<Vec<TechData>, lalrpop_util::ParseError<usize, TechToken, String>> {
        tech::TechsParser::new().parse(tokens)
    }
}
