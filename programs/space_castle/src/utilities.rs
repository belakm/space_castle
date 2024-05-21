use std::ops::{Div, Mul};

/// Converts a `u64` value - in this case the balance of a token account - into
/// an `f32` by using the `decimals` value of its associated mint to get the
/// nominal quantity of a mint stored in that token account
///
/// For example, a token account with a balance of 10,500 for a mint with 3
/// decimals would have a nominal balance of 10.5
pub fn convert_to_float(value: u64, decimals: u8) -> f32 {
    (value as f32).div(f32::powf(10.0, decimals as f32))
}

/// Converts a nominal value - in this case the calculated value `r` - into a
/// `u64` by using the `decimals` value of its associated mint to get the real
/// quantity of the mint that the user will receive
///
/// For example, if `r` is calculated to be 10.5, the real amount of the asset
/// to be received by the user is 10,500
pub fn convert_from_float(value: f32, decimals: u8) -> u64 {
    value.mul(f32::powf(10.0, decimals as f32)) as u64
}

/// Calculates upgrade cost with a growth factor so the upgrading costs a bit more each level
pub fn calculate_upgrade_cost(base_cost: f32, growth_factor: f32, level: u8) -> f32 {
    base_cost * (growth_factor.powf((level - 1) as f32))
}

/// Sums two `(u64, [u64; 4])` tuples where the tuple is meant as `(igt_cost, [resource_costs])`
pub fn sum_costs(base: (u64, [u64; 4]), cost: (u64, [u64; 4])) -> (u64, [u64; 4]) {
    return (
        base.0.saturating_add(cost.0),
        [
            base.1[0].saturating_add(cost.1[0]),
            base.1[1].saturating_add(cost.1[1]),
            base.1[2].saturating_add(cost.1[2]),
            base.1[3].saturating_add(cost.1[3]),
        ],
    );
}

/// Multiplies `(u64, [u64; 4])` tuple where the tuple is meant as `(igt_cost, [resource_costs])`
///
/// # Params
/// * `(igt, [metal, crystal, chemical, fuel])`
/// * `amount` - multiplier
pub fn multiply_costs(base: (u64, [u64; 4]), amount: u64) -> (u64, [u64; 4]) {
    return (
        base.0.saturating_mul(amount),
        [
            base.1[0].saturating_mul(amount),
            base.1[1].saturating_mul(amount),
            base.1[2].saturating_mul(amount),
            base.1[3].saturating_mul(amount),
        ],
    );
}
