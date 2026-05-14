# cp-curve

constant product (x*y=k) amm math primitives. no_std rust.

## api

- `swap_output(reserve_in, reserve_out, amount_in)` — swap, no fee
- `swap_output_with_fee(reserve_in, reserve_out, amount_in, fee_bps)` — swap with fee in bps
- `deposit_amounts(reserve_x, reserve_y, total_lp, amount_x_in, amount_y_in)` — lp deposit, first-deposit via integer sqrt
- `withdraw_amounts(reserve_x, reserve_y, total_lp, lp_burn)` — proportional withdraw
- `integer_sqrt(value)` — u128 newton's method

## errors

`CurveError`: `Overflow`, `ZeroInput`, `EmptyPool`, `InvalidFee`, `InsufficientLpSupply`.

## test

```
cargo test
```
