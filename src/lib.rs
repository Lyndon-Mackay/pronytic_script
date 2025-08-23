use std::{fmt, num::ParseIntError};

use building::{BuildingData, parse_buildings_section};
use goods::{GoodData, parse_goods_section};
use lalrpop_util::lalrpop_mod;
use planet_types::{PlanetTypeData, parse_planet_types_section};
use regex::Regex;
use tech::{TechData, parse_tech_section};
use tracing::*;

use miette::{Diagnostic, NamedSource, SourceSpan};

use thiserror::Error;

use logos::{self, Logos};

use crate::{
    augmentations::{AugmentationData, parse_augmentations},
    shipyard::{ShipyardData, parse_shipyard},
    species_trait::{SpeciesTraitData, parse_species_traits},
};

pub mod augmentations;
pub mod building;
pub mod common;
pub mod goods;
pub mod planet_types;
pub mod shipyard;
pub mod species_trait;
pub mod tech;

/// Placeholder for better syntax errors
#[derive(Error, Debug, Diagnostic)]
#[error("oops!")]
#[diagnostic(code(oops::my::bad), url(docsrs))]
struct SyntaxError {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource<String>,
    // Snippets and highlights can be included in the diagnostic!
    #[label("Problem started here")]
    bad_bit: SourceSpan,
    #[help]
    advice: Option<String>,
}

///Token errors
#[derive(Error, Debug, Diagnostic)]
#[error("I came across an invalid token")]
struct TokenErrorReport {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource<String>,
    // Snippets and highlights can be included in the diagnostic!
    #[label("Problem started here")]
    bad_bit: SourceSpan,
    #[help]
    advice: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

#[derive(Clone, Default, Debug)]
pub struct ParseData {
    pub augmentations: Vec<AugmentationData>,
    pub building_data: Vec<BuildingData>,
    pub goods_data: Vec<GoodData>,
    pub planet_type_data: Vec<PlanetTypeData>,
    pub tech_data: Vec<TechData>,
    pub species_trait: Vec<SpeciesTraitData>,
    pub shipyard: Vec<ShipyardData>,
}

impl ParseData {
    ///Combines ParseData
    pub fn combine(&mut self, mut other: ParseData) {
        self.augmentations.append(&mut other.augmentations);
        self.building_data.append(&mut other.building_data);
        self.goods_data.append(&mut other.goods_data);
        self.planet_type_data.append(&mut other.planet_type_data);
        self.species_trait.append(&mut other.species_trait);
        self.shipyard.append(&mut other.shipyard);
        self.tech_data.append(&mut other.tech_data);
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(error=LexicalError)]
pub enum Token {
    #[token("#buildings")]
    Buildings,
    #[token("#goods")]
    Goods,
    #[token("#tech")]
    Tech,
    #[token("#planet_types")]
    PlanetTypes,

    #[token("#specie_traits")]
    SpecieTraits,
    #[token("#augmentations")]
    Augmentations,

    #[token("#shipyard")]
    Shipyard,

    #[regex(r#"[^#]+"#, |lex| lex.slice().trim_matches('"').to_string())]
    SectionContents(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub main);

pub enum Section {
    Buildings(String),
    Goods(String),
    Tech(String),
    PlanetTypes(String),
    SpecieTraits(String),
    Augmentations(String),
    Shipyard(String),
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
                    let problem = TokenErrorReport {
                        src: NamedSource::new(file_name, input.to_string()),
                        bad_bit: (last).into(),
                        advice: Some("Main token".to_string()),
                    };
                    panic!("{:?}", miette::Error::new(problem));
                }
            },
        };
        let span = lex.span();
        tokens.push((span.start, token, span.end));
    }
    tokens
}

pub fn parse(file_name: &str, contents: &str) -> ParseData {
    let tokens = lex(file_name, contents);

    let main_parse = main::SectionsParser::new().parse(tokens);
    let mut parse_data = ParseData::default();

    match main_parse {
        Ok(sections) => {
            for s in sections {
                match s {
                    Section::Augmentations(s) => parse_data
                        .augmentations
                        .append(&mut parse_augmentations(file_name, &s)),
                    Section::Buildings(b) => parse_data
                        .building_data
                        .append(&mut parse_buildings_section(file_name, &b)),
                    Section::Goods(g) => parse_data
                        .goods_data
                        .append(&mut parse_goods_section(file_name, &g)),
                    Section::Tech(t) => parse_data
                        .tech_data
                        .append(&mut parse_tech_section(file_name, &t)),
                    Section::PlanetTypes(t) => parse_data
                        .planet_type_data
                        .append(&mut parse_planet_types_section(file_name, &t)),
                    Section::SpecieTraits(s) => parse_data
                        .species_trait
                        .append(&mut parse_species_traits(file_name, &s)),
                    Section::Shipyard(s) => {
                        parse_data
                            .shipyard
                            .append(&mut parse_shipyard(file_name, &s));
                    }
                }
            }
        }
        Err(e) => match e {
            lalrpop_util::ParseError::InvalidToken { location } => {
                let problem = SyntaxError {
                    src: NamedSource::new(file_name, contents.to_string()),
                    bad_bit: (location).into(),
                    advice: Some("Skill issue".to_string()),
                };

                panic!("{:?}", miette::Error::new(problem));
            }
            lalrpop_util::ParseError::UnrecognizedEof { .. } => todo!(),
            lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                let problem = SyntaxError {
                    src: NamedSource::new(file_name, contents.to_string()),
                    bad_bit: (token.0, token.2).into(),
                    advice: Some(format!("Expected {} found {}", expected.join(","), token.1)),
                };
                panic!("{:?}", miette::Error::new(problem));
            }
            lalrpop_util::ParseError::ExtraToken { .. } => todo!(),
            lalrpop_util::ParseError::User { .. } => todo!(),
        },
    }
    parse_data
}

/// Pretty prints the lexical error message to try and give the user
/// The clearest indication where the error likely is
fn handle_lexical_errors(
    file_name: &str,
    lexical_error: LexicalError,
    input: &str,
    last: usize,
) -> ! {
    // Needs to be in sync with actual skip tokens, unfortunately
    // cannot be done in const context as macros require string
    let skip_texts: [&str; 2] = [r"//[^\n\r]*", r"[\s\t\f]+"];
    match lexical_error {
        //I have never come across this error, when I do I will figure how to present this error
        LexicalError::InvalidInteger(..) => todo!(),
        LexicalError::InvalidToken => {
            let regexes = skip_texts
                .into_iter()
                .flat_map(Regex::new)
                .collect::<Vec<_>>();

            let error = input
                .char_indices()
                .skip(last)
                .skip_while(|(_, c)| regexes.iter().any(|r| r.is_match(c.to_string().as_str())))
                .map(|(i, _)| i)
                .next()
                .unwrap();
            let problem = TokenErrorReport {
                src: NamedSource::new(file_name, input.to_string()),
                bad_bit: (error).into(),
                advice: Some("I don't have a definition for this word\nNote:File numbers are relative to the '#' sections".to_string()),
            };
            panic!("{:?}", miette::Error::new(problem));
        }
    }
}
