use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use super::{chain::ChainId, token::Token};

/// Type of order: market or limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    /// Execute immediately at best available price.
    Market,
    /// Execute at a specified limit price.
    Limit,
}

/// Represents a user order to swap tokens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    /// Unique order identifier.
    pub id: String,
    /// Address of the order creator.
    pub owner: String,
    /// Token being sold.
    pub sell_token: Token,
    /// Token being bought.
    pub buy_token: Token,
    /// Amount of sell_token offered.
    pub sell_amount: Decimal,
    /// Minimum amount of buy_token expected.
    pub buy_amount: Decimal,
    /// Unix timestamp (seconds) at which the order expires.
    pub expiration: u64,
    /// The type of order (market or limit).
    pub order_type: OrderType,
}

impl Order {
    /// Creates a new order instance.
    pub fn new(
        id: impl Into<String>,
        owner: impl Into<String>,
        sell_token: Token,
        buy_token: Token,
        sell_amount: Decimal,
        buy_amount: Decimal,
        expiration: u64,
        order_type: OrderType,
    ) -> Self {
        Self {
            id: id.into(),
            owner: owner.into(),
            sell_token,
            buy_token,
            sell_amount,
            buy_amount,
            expiration,
            order_type,
        }
    }

    /// Returns true if the order has expired given a current Unix timestamp.
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time >= self.expiration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn sample_token(symbol: &str) -> Token {
        Token::new(
            format!("0x{}", "0".repeat(40)),
            symbol,
            symbol.to_string(),
            18,
            ChainId::EthereumMainnet,
        )
    }

    #[test]
    fn order_serialization_roundtrip() {
        let order = Order::new(
            "1",
            "0x1111111111111111111111111111111111111111",
            sample_token("ETH"),
            sample_token("DAI"),
            Decimal::from(1),
            Decimal::from(1800),
            1_700_000_000,
            OrderType::Limit,
        );
        let json = serde_json::to_string(&order).unwrap();
        let des: Order = serde_json::from_str(&json).unwrap();
        assert_eq!(order, des);
    }

    #[test]
    fn is_expired_checks_correctly() {
        let order = Order::new(
            "2",
            "0x2222222222222222222222222222222222222222",
            sample_token("USDC"),
            sample_token("ETH"),
            Decimal::from(1000),
            Decimal::from(0.5),
            1_600_000_000,
            OrderType::Market,
        );
        assert!(!order.is_expired(1_500_000_000));
        assert!(order.is_expired(1_600_000_000));
    }
}
