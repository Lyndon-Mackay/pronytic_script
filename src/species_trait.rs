use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use rust_decimal::Decimal;

use logos::Logos;

use crate::{
    LexicalError,
    common::{DataParser, GoodConsumes},
};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum Token {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

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
    #[token("icon")]
    Icon,

    #[token("good_id")]
    GoodId,
    #[token("amount")]
    Amount,

    #[token("consumes")]
    Consumes,

    #[token("effects")]
    Effects,

    #[token("growth_rate")]
    GrowthRate,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub species_trait);

#[derive(Clone, Default, Debug)]
pub struct SpeciesTraitData {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub consumes: Vec<GoodConsumes>,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone)]
pub enum Effect {
    GrowthRate(Decimal),
}

#[derive(Debug, Clone)]
pub enum Field {
    Name(String),
    Icon(String),
    Consumes(Vec<GoodConsumes>),
    Effects(Vec<Effect>),
}

impl<'s> DataParser<'s, Token, SpeciesTraitData> for SpeciesTraitData {
    fn parse_tokens(
        tokens: Vec<(usize, Token, usize)>,
    ) -> Result<Vec<SpeciesTraitData>, lalrpop_util::ParseError<usize, Token, String>> {
        species_trait::SpeciesTraitsParser::new().parse(tokens)
    }
}
