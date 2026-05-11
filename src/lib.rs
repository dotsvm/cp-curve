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
    todo!()
}

/// Same as `swap_output` but takes a fee (in basis points) from the input
/// before the swap. The fee stays in the pool.
pub fn swap_output_with_fee(
    reserve_in:  u64,
    reserve_out: u64,
    amount_in:   u64,
    fee_bps:     u16,
) -> Result<u64, CurveError> {
    todo!()
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
    todo!()
}
