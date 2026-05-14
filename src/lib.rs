#![no_std]

/// Errors returnable by every function in this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveError {
    /// An arithmetic operation overflowed even at u128.
    Overflow,
    /// Caller passed a zero amount where a positive amount is required.
    ZeroInput,
    /// Pool reserves are zero and the operation requires a non-empty pool.
    EmptyPool,
    /// fee_bps >= 10_000 (>=100%).
    InvalidFee,
    /// LP burn amount exceeds total LP supply.
    InsufficientLpSupply,
}

/// Returned by `deposit_amounts`: how much of each token was actually used,
/// and how many LP tokens to mint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DepositResult {
    pub amount_x_used: u64,
    pub amount_y_used: u64,
    pub lp_minted:     u64,
}

/// Given pool reserves and an input amount, compute how much of the other
/// token the swapper receives. **No fee.**
///
/// Returns `Err(ZeroInput)` if `amount_in == 0`.
/// Returns `Err(EmptyPool)` if either reserve is zero.
pub fn swap_output(
    reserve_in:  u64,
    reserve_out: u64,
    amount_in:   u64,
) -> Result<u64, CurveError> {
    if amount_in == 0 {
        return Err(CurveError::ZeroInput);
    }

    if reserve_in == 0 || reserve_out == 0 {
        return Err(CurveError::EmptyPool);
    }

    let reserve_in = reserve_in as u128;
    let reserve_out = reserve_out as u128;
    let amount_in = amount_in as u128;

    let numerator = amount_in
        .checked_mul(reserve_out)
        .ok_or(CurveError::Overflow)?;

    let denominator = reserve_in
        .checked_add(amount_in)
        .ok_or(CurveError::Overflow)?;

    let amount_out = numerator / denominator;

    u64::try_from(amount_out).map_err(|_| CurveError::Overflow)
    
}

/// Same as `swap_output` but takes a fee (in basis points) from the input
/// before the swap. The fee stays in the pool.
pub fn swap_output_with_fee(
    reserve_in:  u64,
    reserve_out: u64,
    amount_in:   u64,
    fee_bps:     u16,
) -> Result<u64, CurveError> {
    if amount_in == 0 {
        return Err(CurveError::ZeroInput);
    }

    if reserve_in == 0 || reserve_out == 0 {
        return Err(CurveError::EmptyPool);
    }

    if fee_bps >= 10_000 {
        return Err(CurveError::InvalidFee);
    }

    let reserve_in = reserve_in as u128;
    let reserve_out = reserve_out as u128;
    let amount_in = amount_in as u128;
    let fee_bps = fee_bps as u128;

    let amount_in_with_fee = amount_in
        .checked_mul(10_000 - fee_bps)
        .ok_or(CurveError::Overflow)?;

    let numerator = amount_in_with_fee
        .checked_mul(reserve_out)
        .ok_or(CurveError::Overflow)?;

    let reserve_in_scaled = reserve_in
        .checked_mul(10_000)
        .ok_or(CurveError::Overflow)?;

    let denominator = reserve_in_scaled
        .checked_add(amount_in_with_fee)
        .ok_or(CurveError::Overflow)?;

    let amount_out = numerator / denominator;

    u64::try_from(amount_out).map_err(|_| CurveError::Overflow)
    
}

/// Compute how much of each input token to actually pull, and how many LP
/// tokens to mint, given proposed deposit amounts.
///
/// Handles the first-deposit case (total_lp == 0) via integer sqrt.
pub fn deposit_amounts(
    reserve_x:    u64,
    reserve_y:    u64,
    total_lp:     u64,
    amount_x_in:  u64,
    amount_y_in:  u64,
) -> Result<DepositResult, CurveError> {
    todo!()
}

/// Compute how much of each underlying token to return to the LP for burning
/// `lp_burn` LP tokens.
pub fn withdraw_amounts(
    reserve_x: u64,
    reserve_y: u64,
    total_lp:  u64,
    lp_burn:   u64,
) -> Result<(u64, u64), CurveError> {
    todo!()
}

/// Integer square root, Newton's method on u128.
///
/// Returns the largest `n` such that `n * n <= value`.
pub fn integer_sqrt(value: u128) -> u128 {
    if value == 0 {
        return 0;
    }

    if value < 4 {
        return 1;
    }

    let mut z = value;
    let mut x = value / 2 + 1;

    while x < z {
        z = x;
        x = (value / x + x) / 2;
    }
    z
}


#[cfg(test)]
mod tests {
    use core::u128;

    use super::*;

    #[test]
    fn swap_basic_no_fee() {
        assert_eq!(swap_output(100, 100, 50), Ok(33));
    }

    #[test]
    fn swap_zero_input_error() {
        assert_eq!(swap_output(100, 100, 0), Err(CurveError::ZeroInput));
    }

    #[test]
    fn swap_empty_pool_errors() {
        assert_eq!(swap_output(0, 100, 50), Err(CurveError::EmptyPool));
        assert_eq!(swap_output(100, 0, 50), Err(CurveError::EmptyPool));
    }

    #[test]
    fn swap_big_numbers_need_u128() {
        assert_eq!(swap_output(10_000_000_000, 10_000_000_000, 10_000_000_000), Ok(5_000_000_000));
    }

    #[test]
    fn swap_at_u64_max_does_not_panic() {
        let result = swap_output(u64::MAX, u64::MAX, u64::MAX);
        assert!(result.is_ok());
        assert!(result.unwrap() < u64::MAX);
    }

    #[test]
    fn swap_preserves_invariant() {
        let reserve_x: u64 = 1000;
        let reserve_y: u64 = 1000;
        let amount_in: u64 = 250;

        let amount_out = swap_output(reserve_x, reserve_y, amount_in).unwrap();

        let new_x = reserve_x + amount_in;
        let new_y = reserve_y - amount_out;

        let old_k = (reserve_x as u128) * (reserve_y as u128);
        let new_k = (new_x as u128) * (new_y as u128);

        assert!(new_k >= old_k);
    }

    #[test]
    fn swap_tiny_input() {
        assert_eq!(swap_output(1_000_000, 1_000_000, 1), Ok(0));
    }

    #[test]
    fn swap_drains_almost_all() {
        let amount_out = swap_output(1000, 1000, 1_000_000).unwrap();
        assert_eq!(amount_out, 999);
        assert!(amount_out < 1000);
    }

    #[test]
    fn swap_with_fee_matches_no_fee_when_zero() {
        let no_fee = swap_output(1000, 1000, 250).unwrap();
        let zero_fee = swap_output_with_fee(1000, 1000, 250, 0).unwrap();

        assert_eq!(no_fee, zero_fee);
    }

    #[test]
    fn swap_with_fee_30_bps_basic() {
        assert_eq!(swap_output_with_fee(1000, 1000, 100, 30), Ok(90));
    }

    #[test]
    fn swap_with_fee_invalid_fee_errors() {
        assert_eq!(swap_output_with_fee(1000, 1000, 100, 10_000), Err(CurveError::InvalidFee));
        assert_eq!(swap_output_with_fee(1000, 1000, 100, 10_001), Err(CurveError::InvalidFee));
    }

    #[test]
    fn swap_with_fee_preserves_invariant() {
        let reserve_x: u64 = 1_000_000;
        let reserve_y: u64 = 1_000_000;
        let amount_in: u64 = 250_000;
        let fee_bps: u16 = 30;

        let amount_out = swap_output_with_fee(reserve_x, reserve_y, amount_in, fee_bps).unwrap();

        let new_x = reserve_x + amount_in;
        let new_y = reserve_y - amount_out;

        let old_k = (reserve_x as u128) * (reserve_y as u128);
        let new_k = (new_x as u128) * (new_y as u128);

        assert!(new_k > old_k);
    }

    #[test]
    fn sqrt_zero() {
        assert_eq!(integer_sqrt(0), 0);
    }

    #[test]
    fn sqrt_small() {
        assert_eq!(integer_sqrt(1), 1);
        assert_eq!(integer_sqrt(2), 1);
        assert_eq!(integer_sqrt(3), 1);
        assert_eq!(integer_sqrt(4), 2);
    }

    #[test]
    fn sqrt_perfect_sqaures() {
        assert_eq!(integer_sqrt(9), 3);
        assert_eq!(integer_sqrt(16), 4);
        assert_eq!(integer_sqrt(100), 10);
        assert_eq!(integer_sqrt(1_000_000), 1000);
    }

    #[test]
    fn sqrt_big() {
        assert_eq!(integer_sqrt(1_000_000_000_000_000_000), 1_000_000_000);
    }

    #[test]
    fn sqrt_property() {
        for value in [50u128, 99, 1000, 1_000_001, 12_345_678] {
            let r = integer_sqrt(value);
            assert!(r * r <= value);
            assert!((r + 1) * (r + 1) > value);
        }
    }

    #[test]
    fn sqrt_u128_max_does_not_panic() {
        let r = integer_sqrt(u128::MAX);
        assert_eq!(r, u64::MAX as u128);
    }
}