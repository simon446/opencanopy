//! Dependency-free `core` math used by the control logic.
//!
//! The only transcendental we need is `exp` (for the Tetens saturation-vapour-pressure curve in
//! [`crate::climate_controller`]). `core` has no `f64::exp`, and we deliberately avoid a `libm`
//! dependency so the host test suite stays offline (see `control/Cargo.toml`). This implementation
//! is deterministic and identical on host and target — a virtue for a controller whose VPD bands
//! must match the reference vectors in `docs/vpd-climate-model.md` §1.1 to 4 decimal places.

/// Natural exponential, accurate to ~1e-12 over the input range this firmware uses
/// (SVP exponent for air temps 0–50 °C lands in roughly `[0, 3.1]`).
///
/// Method: range-reduce `x = k·ln2 + r` with `r ∈ [-ln2/2, ln2/2]`, evaluate `exp(r)` with a
/// short Taylor series (the small `|r|` makes it converge fast), then scale by `2^k`.
pub fn exp(x: f64) -> f64 {
    if x == 0.0 {
        return 1.0;
    }
    const LN2: f64 = core::f64::consts::LN_2;
    // Round-to-nearest integer k = round(x / ln2).
    let kf = x / LN2;
    let k = if kf >= 0.0 {
        (kf + 0.5) as i64
    } else {
        (kf - 0.5) as i64
    };
    let r = x - (k as f64) * LN2;

    // Taylor series for exp(r); |r| <= ln2/2 ≈ 0.3466, so ~16 terms is far past f64 precision.
    let mut term = 1.0_f64;
    let mut sum = 1.0_f64;
    let mut n = 1.0_f64;
    while n < 18.0 {
        term *= r / n;
        sum += term;
        n += 1.0;
    }
    sum * pow2i(k)
}

/// Exact `2^k` for integer `k` via repeated multiplication (k is small here, |k| < 12).
fn pow2i(k: i64) -> f64 {
    let mut v = 1.0_f64;
    if k >= 0 {
        let mut i = 0;
        while i < k {
            v *= 2.0;
            i += 1;
        }
    } else {
        let mut i = 0;
        while i > k {
            v *= 0.5;
            i -= 1;
        }
    }
    v
}

/// Clamp a float to an inclusive range. (`f64::clamp` exists in core but panics on NaN bounds;
/// this is the explicit, panic-free version the controllers use on sensor-derived values.)
pub fn clampf(v: f32, lo: f32, hi: f32) -> f32 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn exp_matches_std_over_range() {
        // Compare our core-only exp against std's libm-backed exp across the firmware's domain.
        let mut t = -1.0_f64;
        while t <= 3.2 {
            assert!(
                approx(exp(t), t.exp(), 1e-9),
                "exp({t}) = {} vs std {}",
                exp(t),
                t.exp()
            );
            t += 0.013;
        }
    }

    #[test]
    fn exp_known_points() {
        assert!(approx(exp(0.0), 1.0, 1e-12));
        assert!(approx(exp(1.0), core::f64::consts::E, 1e-10));
        assert!(approx(exp(2.0), 7.389_056_098_930_65, 1e-9));
    }

    #[test]
    fn clampf_bounds() {
        assert_eq!(clampf(5.0, 0.0, 100.0), 5.0);
        assert_eq!(clampf(-3.0, 0.0, 100.0), 0.0);
        assert_eq!(clampf(150.0, 0.0, 100.0), 100.0);
    }
}
