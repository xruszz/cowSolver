use rust_decimal::Decimal;
use crate::models::Order;

/// The `PricingEngine` trait defines how to compute a clearing price for a batch of orders.
/// Implementations may use different strategies such as uniform pricing, volume-weighted
/// pricing, or bespoke solver strategies. A `None` return value indicates that no valid
/// clearing price could be computed (e.g., not enough orders).
pub trait PricingEngine {
    /// Given a slice of orders, compute a single clearing price. Returns `None` if
    /// the orders cannot be priced.
    fn compute_clearing_price(&self, orders: &[Order]) -> Option<Decimal>;
}

pub mod uniform;
pub mod volume_weighted;
