use std::{fmt, str::FromStr};

use lalrpop_util::{ParseError, lalrpop_mod};
use logos::{self, Logos};
use rust_decimal::Decimal;

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

    #[token("good_id")]
    GoodId,
    #[token("amount")]
    Amount,

    #[token("name")]
    Name,
    #[token("icon")]
    Icon,

    #[token("consumes")]
    Consumes,

    #[token("effects")]
    Effects,

    #[token("add_trait")]
    AddTrait,
    #[token("remove_trait")]
    RemoveTrait,
    #[token("star_adapt")]
    StarAdapt,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub augmentations);

#[derive(Clone, Default, Debug)]
pub struct AugmentationData {
    pub id: String,
    pub name: String,
    pub icon: String,

    pub consumes: Vec<GoodConsumes>,
    pub effects: Vec<Effect>,
}

#[derive(Clone, Debug)]
pub enum Effect {
    AdaptStarType,
    AddTrait(String),
    RemoveTrait(String),
}

pub enum Field {
    Name(String),
    Icon(String),
    Effects(Vec<Effect>),
    Consumes(Vec<GoodConsumes>),
}

impl<'s> DataParser<'s, Token, AugmentationData> for AugmentationData {
    fn parse_tokens(
        tokens: Vec<(usize, Token, usize)>,
    ) -> Result<Vec<AugmentationData>, ParseError<usize, Token, String>> {
        augmentations::AugmentationsParser::new().parse(tokens)
    }
}
