use core::f32::consts::PI;

/// Converts degrees to radians.
#[must_use]
pub const fn into_rads(degs: f32) -> f32 {
    degs * PI / 180.0
}

/// Converts radians to degrees.
#[must_use]
pub const fn into_degs(rads: f32) -> f32 {
    rads * 180.0 / PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_rads() {
        const RADS: f32 = super::into_rads(0.0);
        approx::assert_relative_eq!(RADS, 0.0);
        approx::assert_relative_eq!(super::into_rads(45.0), PI / 4.0);
        approx::assert_relative_eq!(super::into_rads(90.0), PI / 2.0);
        approx::assert_relative_eq!(super::into_rads(180.0), PI);
        approx::assert_relative_eq!(super::into_rads(360.0), PI * 2.0);
    }

    #[test]
    fn into_degs() {
        const DEGS: f32 = super::into_degs(0.0);
        approx::assert_relative_eq!(DEGS, 0.0);
        approx::assert_relative_eq!(super::into_degs(PI / 4.0), 45.0);
        approx::assert_relative_eq!(super::into_degs(PI / 2.0), 90.0);
        approx::assert_relative_eq!(super::into_degs(PI), 180.0);
        approx::assert_relative_eq!(super::into_degs(PI * 2.0), 360.0);
    }
}
