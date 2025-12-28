use rust_decimal::Decimal;
use rust_decimal::prelude::*;

/// Normalize an integer amount to a `Decimal` given token decimals.
///
/// # Parameters
/// * `amount` - The raw integer amount (for example, in wei).
/// * `decimals` - The number of decimal places for the token (e.g. 18 for ETH).
///
/// # Returns
/// A `Decimal` representing the normalized amount.
pub fn normalize_amount(amount: u128, decimals: u32) -> Decimal {
    let base = Decimal::from_i128_with_scale(1, decimals as i32);
    Decimal::from_u128(amount).unwrap() / base
}

/// Multiply two `Decimal` values.
pub fn mul(a: Decimal, b: Decimal) -> Decimal {
    a * b
}

/// Compute the slippage between the expected and actual amounts.
///
/// Returns zero if the expected amount is zero.
pub fn slippage(expected: Decimal, actual: Decimal) -> Decimal {
    if expected.is_zero() {
        Decimal::ZERO
    } else {
        (expected - actual) / expected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_mul() {
        let a = Decimal::from(2u32);
        let b = Decimal::from(3u32);
        assert_eq!(mul(a, b), Decimal::from(6u32));
    }

    #[test]
    fn test_slippage() {
        let expected = Decimal::from(10u32);
        let actual = Decimal::from(8u32);
        let slip = slippage(expected, actual);
        assert_eq!(slip, Decimal::from(2u32) / Decimal::from(10u32));
    }

    #[test]
    fn test_normalize() {
        let amount: u128 = 1_000_000_000_000_000_000;
        let normalized = normalize_amount(amount, 18);
        assert_eq!(normalized, Decimal::ONE);
    }
}
