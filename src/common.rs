use lalrpop_util::ParseError;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug, Clone)]
pub struct GoodConsumes {
    pub id: String,
    pub amount: Decimal,
}

pub trait DataParser<'s> {
    type Token
    where
        Self::Token: Sized;

    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<Self>, ParseError<usize, Self::Token, String>>;
}

#[derive(Clone, Default, Debug)]
pub struct Temperature {
    kelvin: Decimal,
}

impl Temperature {
    pub fn from_celsius(temp: Decimal) -> Self {
        Temperature {
            kelvin: temp + dec!(273.15),
        }
    }
    pub fn from_kelvin(temp: Decimal) -> Self {
        Temperature { kelvin: temp }
    }

    pub fn celsius(&self) -> Decimal {
        (self.kelvin - dec!(273.15)).trunc_with_scale(2)
    }
    pub fn kelvin(&self) -> Decimal {
        self.kelvin.trunc_with_scale(2)
    }
}

#[derive(Clone, Debug, Default)]
pub struct GoodAbundance {
    pub id: String,
    pub mean: Decimal,
    pub std_dev: Decimal,
}
