use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use miette::NamedSource;
use rust_decimal::Decimal;
use tracing::error;

use logos::{self, Logos};

use crate::{LexicalError, SyntaxError, handle_lexical_errors};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum Token {
    #[token("=")]
    Equal,

    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,

    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[regex(r"(-?\d+\.?\d*)", |lex| Decimal::from_str(lex.slice()).expect("parsed_decimal"), priority = 4)]
    DecimalNumber(Decimal),

    #[token("icon")]
    Icon,

    #[token("name")]
    Name,

    #[token("buy_value")]
    BuyValue,
    #[token("sell_value")]
    SellValue,

    #[token("good_type")]
    GoodType,

    #[token("public")]
    Public,
    #[token("private")]
    Private,
    #[token("tender")]
    Tender,

    #[token("hardcoded_id")]
    HardcodedId,

    #[token("consumption_type")]
    ConsumptionType,

    #[token("prosperity_bonus")]
    ProsperityBonus,

    #[token("none")]
    None,
    #[token("amenity")]
    Amenity,
    #[token("survival")]
    Survival,
    #[token("essential")]
    Essential,

    #[token("magnetosphere")]
    Magnetosphere,
    #[token("atmosphere")]
    Atmosphere,
    #[token("temperature")]
    Temperature,
    #[token("water")]
    Water,
    #[token("breathability")]
    Breathability,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub goods);

#[derive(Clone, Default, Debug)]
pub enum GoodType {
    #[default]
    Public,
    Private,
    Tender,
}

#[derive(Clone, Debug, Default)]
pub struct GoodData {
    pub id: String,
    pub hardcoded_id: Option<u8>,
    pub icon: String,
    pub name: String,
    pub good_type: GoodType,
    pub consumption_type: ConsumptionType,
    pub prosperity_bonus: Decimal,
    pub buy_value: Decimal,
    pub sell_value: Decimal,
}

#[derive(Clone, Default, Debug)]
pub enum ConsumptionType {
    #[default]
    None,
    //Goods that the population likes to have
    Amenity,
    ///Needed for harsh enviroments, buying costs purchasing power without giving any bonus to prosperity
    Survival(SurvivalConditions),
    ///Esential is stuff like food absolutely need no matter
    ///Where you are
    Essential,
}

///For triggerering when the good is needed these are thresholds on
/// when to stop
#[derive(Clone, Default, Debug, PartialEq)]
pub struct SurvivalConditions {
    pub magnetosphere: Option<Decimal>,
    pub atmosphere: Option<Decimal>,
    pub temperature: Option<Decimal>,
    pub water: Option<Decimal>,
    pub breathability: Option<Decimal>,
}

pub enum Field {
    Icon(String),
    Name(String),
    BuyValue(Decimal),
    SellValue(Decimal),
    GoodType(GoodType),
    HardcodedId(u8),
    ConsumptionType(ConsumptionType),
    ProsperityBonus(Decimal),
}

pub enum SurvivalField {
    Magnetosphere(Decimal),
    Atmosphere(Decimal),
    Temperature(Decimal),
    Water(Decimal),
    Breathability(Decimal),
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

pub(super) fn parse_goods_section(file_name: &str, input: &str) -> Vec<GoodData> {
    let tokens = lex(file_name, input);
    let goods_parse = goods::GoodsParser::new().parse(tokens);

    match goods_parse {
        Ok(g) => g,
        Err(e) => match e {
            lalrpop_util::ParseError::InvalidToken { location } => {
                let problem = SyntaxError {
                    src: NamedSource::new(file_name, input.to_string()),
                    bad_bit: (location).into(),
                    advice: Some("Skill issue".to_string()),
                };

                panic!("{:?}", miette::Error::new(problem));
            }
            lalrpop_util::ParseError::UnrecognizedEof {
                location: _,
                expected: _,
            } => todo!(),
            lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                let problem = SyntaxError {
                    src: NamedSource::new(file_name, input.to_string()),
                    bad_bit: (token.0, token.2).into(),
                    advice: Some(format!("Expected {} found {}", expected.join(","), token.1)),
                };
                panic!("{:?}", miette::Error::new(problem));
            }
            lalrpop_util::ParseError::ExtraToken { token: _ } => todo!(),
            lalrpop_util::ParseError::User { error: _ } => todo!(),
        },
    }
}
