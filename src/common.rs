use lalrpop_util::ParseError;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct GoodConsumes {
    pub id: String,
    pub amount: Decimal,
}

pub trait DataParser<'s, Token, Data> {
    fn parse_tokens(
        tokens: Vec<(usize, Token, usize)>,
    ) -> Result<Vec<Data>, ParseError<usize, Token, String>>;
}
