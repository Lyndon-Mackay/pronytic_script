use std::{fmt, str::FromStr};

use lalrpop_util::lalrpop_mod;
use rust_decimal::Decimal;

use logos::{self, Logos};

use crate::{LexicalError, common::DataParser};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum GoodToken {
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
    #[token("prosperity_cost")]
    ProsperityCost,

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
    #[token("served_step")]
    ServedStep,
    #[token("lack_of_service_penalty")]
    LackServicePenalty,
}

impl fmt::Display for GoodToken {
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
    pub prosperity_cost: Decimal,
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
#[derive(Clone, Debug, PartialEq)]
pub struct SurvivalConditions {
    pub magnetosphere: Option<Decimal>,
    pub atmosphere: Option<Decimal>,
    pub temperature: Option<Decimal>,
    pub water: Option<Decimal>,
    pub breathability: Option<Decimal>,

    ///Each step is considered served by natural conditions
    pub served_step: Decimal,
    pub lack_of_service_penalty: Decimal,
}

impl Default for SurvivalConditions {
    fn default() -> Self {
        Self {
            magnetosphere: Default::default(),
            atmosphere: Default::default(),
            temperature: Default::default(),
            water: Default::default(),
            breathability: Default::default(),
            served_step: Decimal::ONE,
            lack_of_service_penalty: Decimal::ZERO,
        }
    }
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
    ProsperityCost(Decimal),
}

pub enum SurvivalField {
    Magnetosphere(Decimal),
    Atmosphere(Decimal),
    Temperature(Decimal),
    Water(Decimal),
    Breathability(Decimal),
    ServedStep(Decimal),
    LackServicePenalty(Decimal),
}

impl<'s> DataParser<'s, GoodToken, GoodData> for GoodData {
    fn parse_tokens(
        tokens: Vec<(usize, GoodToken, usize)>,
    ) -> Result<Vec<GoodData>, lalrpop_util::ParseError<usize, GoodToken, String>> {
        goods::GoodsParser::new().parse(tokens)
    }
}
