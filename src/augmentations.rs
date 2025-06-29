use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use logos::{self, Logos};
use rust_decimal::Decimal;

use crate::{LexicalError, SyntaxError, handle_lexical_errors};
use miette::NamedSource;

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

#[derive(Debug, Clone)]
pub struct GoodConsumes {
    pub id: String,
    pub amount: Decimal,
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

pub(super) fn parse_augmentations(file_name: &str, input: &str) -> Vec<AugmentationData> {
    let tokens = lex(file_name, input);
    let species_trait_parse = augmentations::AugmentationsParser::new().parse(tokens);
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
