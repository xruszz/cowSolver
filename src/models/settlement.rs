use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

use super::{
    chain::ChainId,
    order::{Order, OrderType},
    token::Token,
};

/// Represents a token transfer between two parties as part of settlement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    pub from: String,
    pub to: String,
    pub token: Token,
    pub amount: Decimal,
}

/// Represents the settlement of one or more orders at a uniform clearing price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settlement {
    /// Uniform clearing price used for matched orders.
    pub clearing_price: Decimal,
    /// Orders that were settled.
    pub orders: Vec<Order>,
    /// Resulting token transfers required to settle the orders.
    pub transfers: Vec<Transfer>,
    /// Chain on which settlement occurs.
    pub chain_id: ChainId,
}

impl Settlement {
    /// Creates a new settlement instance.
    pub fn new(
        clearing_price: Decimal,
        orders: Vec<Order>,
        transfers: Vec<Transfer>,
        chain_id: ChainId,
    ) -> Self {
        Self {
            clearing_price,
            orders,
            transfers,
            chain_id,
        }
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

    fn sample_order(id: &str) -> Order {
        Order::new(
            id,
            "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
            sample_token("ETH"),
            sample_token("DAI"),
            Decimal::from(1),
            Decimal::from(1800),
            1_700_000_000,
            OrderType::Limit,
        )
    }

    #[test]
    fn settlement_serialization_roundtrip() {
        let orders = vec![sample_order("1")];
        let transfers = vec![Transfer {
            from: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".into(),
            to: "0x1234123412341234123412341234123412341234".into(),
            token: sample_token("DAI"),
            amount: Decimal::from(1800),
        }];
        let settlement = Settlement::new(
            Decimal::from(1800),
            orders.clone(),
            transfers.clone(),
            ChainId::EthereumMainnet,
        );
        let json = serde_json::to_string(&settlement).unwrap();
        let des: Settlement = serde_json::from_str(&json).unwrap();
        assert_eq!(settlement, des);
    }
}
