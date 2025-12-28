use super::{AmmRouter, SwapPath};
use crate::models::{order::Order, token::Token};
use rust_decimal::Decimal;

/// Router implementation for Uniswap-based AMMs.
pub struct UniswapRouter;

impl AmmRouter for UniswapRouter {
    fn find_best_route(&self, _order: &Order) -> Option<SwapPath> {
        // TODO: query on-chain liquidity pools for the optimal path. For now, return None.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{order::{Order, OrderType}, token::Token, chain::ChainId};
    use rust_decimal::prelude::*;

    fn dummy_order() -> Order {
        let sell_token = Token::new("A", "A", 18, ChainId::EthereumMainnet);
        let buy_token = Token::new("B", "B", 18, ChainId::EthereumMainnet);
        Order::new(
            1,
            "Alice",
            sell_token,
            buy_token,
            Decimal::from_i64(100).unwrap(),
            Decimal::from_i64(100).unwrap(),
            0,
            OrderType::Market,
        )
    }

    #[test]
    fn test_uniswap_router_no_route() {
        let router = UniswapRouter;
        let order = dummy_order();
        assert!(router.find_best_route(&order).is_none());
    }
}
