use std::fmt;

use lalrpop_util::lalrpop_mod;

use logos::{self, Logos};
use miette::NamedSource;

use crate::{LexicalError, SyntaxError, lex};

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
pub enum Token {
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

impl fmt::Display for Token {
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

pub(super) fn parse_tech_section(file_name: &str, input: &str) -> Vec<TechData> {
    let tokens = lex::<Token>(file_name, input);
    let tech_parse = tech::TechsParser::new().parse(tokens);
    match tech_parse {
        Ok(t) => t,
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
