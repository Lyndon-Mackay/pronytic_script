use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct GoodConsumes {
    pub id: String,
    pub amount: Decimal,
}
