use super::{AmmRouter, SwapPath};
use crate::models::{order::Order, token::Token};
use rust_decimal::Decimal;

/// Router implementation for Curve-based AMMs.
pub struct CurveRouter;

impl AmmRouter for CurveRouter {
    fn find_best_route(&self, _order: &Order) -> Option<SwapPath> {
        // TODO: compute best route across Curve pools
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
            3,
            "Carol",
            sell_token,
            buy_token,
            Decimal::from_i64(300).unwrap(),
            Decimal::from_i64(300).unwrap(),
            0,
            OrderType::Market,
        )
    }

    #[test]
    fn test_curve_router_no_route() {
        let router = CurveRouter;
        let order = dummy_order();
        assert!(router.find_best_route(&order).is_none());
    }
}
