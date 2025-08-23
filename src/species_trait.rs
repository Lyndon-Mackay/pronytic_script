use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use miette::NamedSource;
use rust_decimal::Decimal;

use logos::Logos;

use crate::{LexicalError, SyntaxError, common::GoodConsumes, handle_lexical_errors};

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

fn lex(file_name: &str, input: &str) -> Vec<(usize, Token, usize)> {
    let mut lex = Token::lexer(input);
    let mut tokens = Vec::new();
    while let Some(tok) = lex.next() {
        let token = match tok {
            Ok(t) => t,
            Err(e) => match e {
                LexicalError::InvalidInteger(_parse_int_error) => todo!(),
                LexicalError::InvalidToken => {
                    let last: usize = tokens.last().map(|(_, _, x)| *x).unwrap_or_default();

                    handle_lexical_errors(file_name, e, input, last);
                }
            },
        };
        let span = lex.span();
        tokens.push((span.start, token, span.end));
    }
    tokens
}

pub(super) fn parse_species_traits(file_name: &str, input: &str) -> Vec<SpeciesTraitData> {
    let tokens = lex(file_name, input);
    let species_trait_parse = species_trait::SpeciesTraitsParser::new().parse(tokens);
    match species_trait_parse {
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
