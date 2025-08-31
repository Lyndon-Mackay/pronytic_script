use std::fmt::{self, Display};

use building::BuildingData;
use goods::GoodData;
use lalrpop_util::lalrpop_mod;
use planet_types::PlanetTypeData;
use regex::Regex;
use tech::TechData;
use tracing::*;

use miette::{Diagnostic, NamedSource, SourceSpan};

use thiserror::Error;

use logos::{self, Logos};

use crate::{
    asteroid_mining::AsteroidMiningData, augmentations::AugmentationData, common::DataParser,
    orbital::OrbitalData, shipyard::ShipyardData, species_trait::SpeciesTraitData,
    stapledon_swarm::StapledonSwarmData,
};

use asteroid_mining::AsteroidToken;
use augmentations::AugmentationToken;
use building::BuildingToken;
use goods::GoodToken;
use orbital::OrbitalToken;
use planet_types::PlanetTypeToken;
use shipyard::ShipyardToken;
use species_trait::SpeciesToken;
use stapledon_swarm::StapledonToken;
use tech::TechToken;

pub mod asteroid_mining;
pub mod augmentations;
pub mod building;
pub mod common;
pub mod goods;
pub mod orbital;
pub mod planet_types;
pub mod shipyard;
pub mod species_trait;
pub mod stapledon_swarm;
pub mod stellar_system;
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
    #[default]
    InvalidToken,
}

macro_rules! create_parse_data {
  ({ $( pub $field:ident : $ty:ty ),* $(,)? }) => {
    ///This is the stored results from a given string of data
    ///typically a file
    #[derive(Clone, Default, Debug)]
    pub struct ParseData { $( pub $field: $ty, )* }

    impl ParseData {
       pub fn combine(&mut self, mut other: ParseData) {
          $( self.$field.append(&mut other.$field); )*
       }
    }
  }
}

create_parse_data!({
    pub asteroid_mining: Vec<AsteroidMiningData>,
    pub augmentations: Vec<AugmentationData>,
    pub building_data: Vec<BuildingData>,
    pub goods_data: Vec<GoodData>,
    pub orbital_data: Vec<OrbitalData>,
    pub planet_type_data: Vec<PlanetTypeData>,
    pub species_trait: Vec<SpeciesTraitData>,
    pub shipyard: Vec<ShipyardData>,
    pub stapledon:Vec<StapledonSwarmData>,
    pub tech_data: Vec<TechData>,
});

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

    #[token("#asteroid_mining")]
    AsteroidMining,

    #[token("#orbital")]
    Orbital,

    #[token("#stapledon_swarm")]
    Stapledon,

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
    AsteroidMining(String),
    Augmentations(String),
    Buildings(String),
    Goods(String),
    Orbital(String),
    PlanetTypes(String),
    SpecieTraits(String),
    Shipyard(String),
    Stapledon(String),
    Tech(String),
}

fn lex<'s, T>(file_name: &str, input: &'s str) -> Vec<(usize, T, usize)>
where
    T: Logos<'s, Source = str, Error = LexicalError>,
    T::Extras: Default,
{
    let mut lex = T::lexer(input);
    let mut tokens = Vec::new();
    while let Some(tok) = lex.next() {
        let token = match tok {
            Ok(t) => t,
            Err(e) => match e {
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

fn parse_section<'s, Token, Data>(file_name: &'s str, input: &'s str) -> Vec<Data>
where
    Data: DataParser<'s, Token, Data>,
    Token: Logos<'s, Source = str, Error = LexicalError> + Display,
    Token::Extras: Default,
{
    let tokens = lex::<Token>(file_name, input);
    let data_parse = Data::parse_tokens(tokens);

    match data_parse {
        Ok(list) => list,
        //TODO's are on errors because I have not triggered them and don't know how to
        // if I come across them I will give them proper implementations
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

pub fn parse(file_name: &str, contents: &str) -> ParseData {
    let tokens = lex::<Token>(file_name, contents);

    let main_parse = main::SectionsParser::new().parse(tokens);
    let mut parse_data = ParseData::default();

    match main_parse {
        Ok(sections) => {
            for s in sections {
                match s {
                    Section::AsteroidMining(s) => {
                        parse_data.asteroid_mining.append(&mut parse_section::<
                            AsteroidToken,
                            AsteroidMiningData,
                        >(
                            file_name, &s
                        ))
                    }
                    Section::Augmentations(s) => {
                        parse_data.augmentations.append(&mut parse_section::<
                            AugmentationToken,
                            AugmentationData,
                        >(
                            file_name, &s
                        ))
                    }
                    Section::Buildings(s) => parse_data.building_data.append(&mut parse_section::<
                        BuildingToken,
                        BuildingData,
                    >(
                        file_name, &s
                    )),
                    Section::Goods(s) => {
                        parse_data
                            .goods_data
                            .append(&mut parse_section::<GoodToken, GoodData>(file_name, &s))
                    }
                    Section::Orbital(o) => parse_data.orbital_data.append(&mut parse_section::<
                        OrbitalToken,
                        OrbitalData,
                    >(
                        file_name, &o
                    )),
                    Section::PlanetTypes(s) => {
                        parse_data.planet_type_data.append(&mut parse_section::<
                            PlanetTypeToken,
                            PlanetTypeData,
                        >(
                            file_name, &s
                        ))
                    }
                    Section::Shipyard(s) => {
                        parse_data.shipyard.append(
                            &mut parse_section::<ShipyardToken, ShipyardData>(file_name, &s),
                        );
                    }
                    Section::SpecieTraits(s) => {
                        parse_data.species_trait.append(&mut parse_section::<
                            SpeciesToken,
                            SpeciesTraitData,
                        >(
                            file_name, &s
                        ))
                    }
                    Section::Stapledon(s) => parse_data.stapledon.append(&mut parse_section::<
                        StapledonToken,
                        StapledonSwarmData,
                    >(
                        file_name, &s
                    )),
                    Section::Tech(s) => {
                        parse_data
                            .tech_data
                            .append(&mut parse_section::<TechToken, TechData>(file_name, &s))
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
