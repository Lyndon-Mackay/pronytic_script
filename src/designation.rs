use std::fmt;

use lalrpop_util::lalrpop_mod;
use logos::Logos;

use crate::{LexicalError, common::DataParser};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum DesignationToken {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(\d+)", |lex|lex.slice().parse::<u8>().expect("parsing u8"), priority = 5)]
    Number(u8),

    #[token("=")]
    Equal,

    #[token("(")]
    RightBracket,
    #[token(")")]
    LeftBracket,

    #[token("name")]
    Name,
    #[token("description")]
    Description,

    #[token("building_limit")]
    BuildingLimit,
    #[token("Unlimited")]
    Unlimited,
    #[token("limited")]
    Limited,

    #[token("housing")]
    Housing,
    #[token("managend")]
    Managed,
    #[token("unmanaged")]
    Unmanaged,
}

impl fmt::Display for DesignationToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub designation);

#[derive(Clone, Default, Debug)]
pub enum BuildingLimit {
    #[default]
    Unlimited,
    Limited(u8),
}

#[derive(Clone, Default, Debug)]
pub enum Housing {
    #[default]
    Managed,
    Unmanaged,
}

#[derive(Clone, Default, Debug)]
pub struct DesignationData {
    pub id: String,

    pub name: String,
    pub description: String,

    pub building_limit: BuildingLimit,
    pub housing: Housing,
}

pub enum Field {
    Name(String),
    Description(String),
    Housing(Housing),
    BuildingLimit(BuildingLimit),
}

impl<'s> DataParser<'s> for DesignationData {
    type Token = DesignationToken;

    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<Self>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        designation::DesignationDataParser::new().parse(tokens)
    }
}
