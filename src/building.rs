use std::{fmt, str::FromStr};

use rust_decimal::Decimal;

use lalrpop_util::lalrpop_mod;
use miette::NamedSource;

use crate::{LexicalError, SyntaxError, handle_lexical_errors};
use logos::{self, Logos};

#[derive(Clone, Debug, Default)]
pub struct CustomGood {
    pub id: String,
    pub amount: Decimal,
}
/// Building data to send to game
/// this is only made for serialisation
/// actual data structure in game is different
#[derive(Clone, Debug)]
pub struct BuildingData {
    pub id: String,
    pub name: String,

    pub planet_filters: Vec<String>,

    pub initial: bool,
    pub unique: bool,

    pub energy: Decimal,

    pub costs: Vec<CustomGood>,
    pub private_costs: Decimal,
    pub consumes: Vec<CustomGood>,
    pub produces: Vec<CustomGood>,

    pub housing: u64,
    pub workers: u64,

    pub private_sector: bool,

    pub magnetosphere_equilibrium: Decimal,
    pub atmosphere_equilibrium: Decimal,

    pub temperature_change: Decimal,
    pub water_change: Decimal,
    pub breathable_change: Decimal,

    pub tech_needed: Option<String>,
    pub upgrades_from: Option<String>,
}

impl Default for BuildingData {
    fn default() -> Self {
        BuildingData {
            id: "".to_string(),
            name: "".to_string(),
            planet_filters: Vec::new(),
            initial: false,
            unique: false,
            energy: Decimal::ZERO,

            costs: Vec::new(),
            private_costs: Decimal::ZERO,
            consumes: Vec::new(),
            produces: Vec::new(),

            housing: 0,
            workers: 0,

            private_sector: false,

            magnetosphere_equilibrium: Decimal::ZERO,
            atmosphere_equilibrium: Decimal::ZERO,
            temperature_change: Decimal::ZERO,
            water_change: Decimal::ZERO,
            breathable_change: Decimal::ZERO,
            tech_needed: None,
            upgrades_from: None,
        }
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum Token {
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("=")]
    Equal,

    #[token("[")]
    LeftSquare,
    #[token("]")]
    RightSquare,

    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,

    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(-?\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[token("id")]
    Id,
    #[token("name")]
    Name,

    #[token("build_planets")]
    PlanetFilters,

    #[token("initial")]
    Initial,
    #[token("unique")]
    Unique,

    #[token("energy")]
    Energy,

    #[token("private_cost")]
    PrivateCosts,

    #[token("costs")]
    Costs,
    #[token("consumes")]
    Consumes,
    #[token("produces")]
    Produces,

    #[token("housing")]
    Housing,
    #[token("workers")]
    Workers,

    #[token("private_sector")]
    PrivateSector,

    #[token("magnetosphere_equilibrium")]
    MagnetosphereEquilibrium,

    #[token("atmosphere_equilibrium")]
    AtmosphereEquilibrium,

    #[token("temperature_change")]
    TemperatureChange,

    #[token("water_change")]
    WaterChange,

    #[token("breathable_change")]
    BreathableChange,

    #[token("tech_needed")]
    TechNeeded,
    #[token("upgrades_from")]
    UpgradesFrom,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub buildings);
pub enum Field {
    Name(String),
    PlanetFilters(Vec<String>),
    Initial(bool),
    Unique(bool),
    Energy(Decimal),
    PrivateCosts(Decimal),
    Costs(Vec<CustomGood>),
    Consumes(Vec<CustomGood>),
    Produces(Vec<CustomGood>),
    Housing(u64),
    Workers(u64),
    PrivateSector(bool),
    Magnetosphere(Decimal),
    Atmosphere(Decimal),
    TemperatureChange(Decimal),
    WaterChange(Decimal),
    BreathableChange(Decimal),
    TechNeeded(String),
    UpgradesFrom(String),
}

fn lex(file_name: &str, input: &str) -> Vec<(usize, Token, usize)> {
    let mut lex = Token::lexer(input);
    let mut tokens = Vec::new();
    while let Some(tok) = lex.next() {
        let token = match tok {
            Ok(t) => t,
            Err(e) => match e {
                LexicalError::InvalidInteger(..) => todo!(),
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
pub(super) fn parse_buildings_section(file_name: &str, input: &str) -> Vec<BuildingData> {
    let tokens = lex(file_name, input);
    let buildings_parse = buildings::BuildingsParser::new().parse(tokens);
    match buildings_parse {
        Ok(b) => b,
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
