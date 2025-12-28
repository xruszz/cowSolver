use rust_decimal::Decimal;
use crate::models::token::Token;
use crate::models::order::Order;

/// Represents a possible AMM swap path including hops and estimated output amount.
#[derive(Debug, Clone)]
pub struct SwapPath {
    pub hops: Vec<Token>,
    pub estimated_out: Decimal,
}

/// Trait for AMM routing implementations. Given an order, produces the best swap path if available.
pub trait AmmRouter {
    /// Computes the optimal multi-hop path for a given order. Returns `None` if no route available.
    fn find_best_route(&self, order: &Order) -> Option<SwapPath>;
}

pub mod uniswap;
pub mod balancer;
pub mod curve;
