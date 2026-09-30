//! Floating-point functions for `no_std` builds.
//!
//! `core` has no `powf`, `sin`, `cbrt`, ... so without the `std` feature the
//! crate calls them through [`FloatBackend`], whose implementation for `f64`
//! forwards to functions provided by a *backend* at link time:
//!
//! - the `libm` feature registers [`Libm`];
//! - or leave `libm` disabled, implement [`FloatBackend`] and register it with
//!   [`set_float_backend!`](crate::set_float_backend).
//!
//! Exactly one backend must be registered in the final binary. Without one,
//! linking fails with undefined `__material_colors_float_v1_*` symbols; with
//! two, linking fails with duplicate ones. Libraries depending on this crate
//! should never register a backend themselves.
//!
//! ```ignore
//! // Cargo.toml: material-colors = { version = "...", default-features = false }
//! struct HardwareFloats;
//!
//! impl material_colors::utils::no_std::FloatBackend for HardwareFloats {
//!     fn powf(x: f64, n: f64) -> f64 { /* ... */ }
//!     fn sqrt(x: f64) -> f64 { /* ... */ }
//!     fn cbrt(x: f64) -> f64 { /* ... */ }
//!     fn exp(x: f64) -> f64 { /* ... */ }
//!     fn ln(x: f64) -> f64 { /* ... */ }
//!     fn sin(x: f64) -> f64 { /* ... */ }
//!     fn cos(x: f64) -> f64 { /* ... */ }
//!     fn atan2(y: f64, x: f64) -> f64 { /* ... */ }
//! }
//!
//! material_colors::set_float_backend!(HardwareFloats);
//! ```

const SIGN: u64 = 1 << 63;

/// Floating-point functions `core` doesn't provide.
///
/// Only the first eight methods are required. The others have defaults built
/// on `core` or on the required ones; override them when your platform has
/// something faster or more precise (for example a hardware fused
/// multiply-add).
pub trait FloatBackend {
    /// `x` raised to the power `n`.
    fn powf(x: f64, n: f64) -> f64;

    fn sqrt(x: f64) -> f64;

    fn cbrt(x: f64) -> f64;

    /// `e` raised to the power `x`.
    fn exp(x: f64) -> f64;

    /// Natural logarithm.
    fn ln(x: f64) -> f64;

    fn sin(x: f64) -> f64;

    fn cos(x: f64) -> f64;

    /// Four-quadrant arctangent of `y / x`.
    fn atan2(y: f64, x: f64) -> f64;

    /// `x * a + b`. The default is **not** fused (two roundings); override it
    /// if a single-rounding FMA is available.
    fn mul_add(x: f64, a: f64, b: f64) -> f64 {
        x * a + b
    }

    fn powi(x: f64, n: i64) -> f64 {
        Self::powf(x, n as f64)
    }

    fn hypot(x: f64, y: f64) -> f64 {
        Self::sqrt(x * x + y * y)
    }

    /// `ln(1 + x)`. The default loses precision for `x` close to 0.
    fn ln_1p(x: f64) -> f64 {
        Self::ln(1.0 + x)
    }

    /// `exp(x) - 1`. The default loses precision for `x` close to 0.
    fn exp_m1(x: f64) -> f64 {
        Self::exp(x) - 1.0
    }

    fn abs(x: f64) -> f64 {
        f64::from_bits(x.to_bits() & !SIGN)
    }

    fn floor(x: f64) -> f64 {
        let truncated = trunc(x);

        if truncated > x { truncated - 1.0 } else { truncated }
    }

    fn ceil(x: f64) -> f64 {
        let truncated = trunc(x);

        if truncated < x { truncated + 1.0 } else { truncated }
    }

    /// Rounds half away from zero, like `f64::round`.
    fn round(x: f64) -> f64 {
        let truncated = trunc(x);

        if Self::abs(x - truncated) >= 0.5 {
            truncated + copysign(1.0, x)
        } else {
            truncated
        }
    }
}

/// Rounds toward zero, keeping the sign of zero. Values with `|x| >= 2^52`
/// (and infinities, NaN) are already integral and returned unchanged.
fn trunc(x: f64) -> f64 {
    if !(f64::from_bits(x.to_bits() & !SIGN) < 4_503_599_627_370_496.0) {
        return x;
    }

    copysign(x as i64 as f64, x)
}

fn copysign(magnitude: f64, sign: f64) -> f64 {
    f64::from_bits((magnitude.to_bits() & !SIGN) | (sign.to_bits() & SIGN))
}

/// Generates, from one list, everything whose signatures must agree: the
/// [`FloatExt`] trait, the `extern` declarations, the `impl FloatExt for f64`
/// that calls them, and the `set_float_backend!` macro that defines them.
///
/// The first token is `$`, so the generated macro can use its own
/// metavariables (`$d backend` becomes `$backend`).
macro_rules! float_functions {
    ($d:tt $($name:ident(self $(, $arg:ident: $arg_ty:ty)*) -> $ret:ty => $symbol:ident;)*) => {
        /// Float methods for `no_std` builds, shadowing the `std` ones.
        #[allow(dead_code)]
        pub trait FloatExt {
            $(
                #[must_use]
                fn $name(self $(, $arg: $arg_ty)*) -> $ret;
            )*
        }

        // Defined by `set_float_backend!` from the same list, so the
        // signatures match and calling them is safe.
        unsafe extern "Rust" {
            $(safe fn $symbol(value: f64 $(, $arg: $arg_ty)*) -> $ret;)*
        }

        impl FloatExt for f64 {
            $(
                #[inline]
                fn $name(self $(, $arg: $arg_ty)*) -> $ret {
                    $symbol(self $(, $arg)*)
                }
            )*
        }

        /// Registers a [`FloatBackend`](crate::utils::no_std::FloatBackend)
        /// as the crate's floating-point implementation.
        ///
        /// Invoke it exactly once, at module level, in the final binary, and
        /// only when the `libm` feature is disabled.
        #[macro_export]
        macro_rules! set_float_backend {
            ($d backend:ty) => {
                $(
                    #[unsafe(no_mangle)]
                    fn $symbol(value: f64 $(, $arg: $arg_ty)*) -> $ret {
                        <$d backend as $d crate::utils::no_std::FloatBackend>::$name(value $(, $arg)*)
                    }
                )*
            };
        }
    };
}

float_functions! {
    $
    abs(self) -> f64 => __material_colors_float_v1_abs;
    mul_add(self, a: f64, b: f64) -> f64 => __material_colors_float_v1_mul_add;
    powf(self, n: f64) -> f64 => __material_colors_float_v1_powf;
    powi(self, n: i64) -> f64 => __material_colors_float_v1_powi;
    cos(self) -> f64 => __material_colors_float_v1_cos;
    sin(self) -> f64 => __material_colors_float_v1_sin;
    cbrt(self) -> f64 => __material_colors_float_v1_cbrt;
    ln(self) -> f64 => __material_colors_float_v1_ln;
    ln_1p(self) -> f64 => __material_colors_float_v1_ln_1p;
    exp(self) -> f64 => __material_colors_float_v1_exp;
    exp_m1(self) -> f64 => __material_colors_float_v1_exp_m1;
    round(self) -> f64 => __material_colors_float_v1_round;
    ceil(self) -> f64 => __material_colors_float_v1_ceil;
    floor(self) -> f64 => __material_colors_float_v1_floor;
    sqrt(self) -> f64 => __material_colors_float_v1_sqrt;
    hypot(self, y: f64) -> f64 => __material_colors_float_v1_hypot;
    atan2(self, x: f64) -> f64 => __material_colors_float_v1_atan2;
}

/// The [`libm`](https://github.com/rust-lang/libm) backend, registered by the
/// `libm` feature.
#[cfg(feature = "libm")]
pub struct Libm;

#[cfg(feature = "libm")]
impl FloatBackend for Libm {
    fn powf(x: f64, n: f64) -> f64 {
        libm::pow(x, n)
    }

    fn sqrt(x: f64) -> f64 {
        libm::sqrt(x)
    }

    fn cbrt(x: f64) -> f64 {
        libm::cbrt(x)
    }

    fn exp(x: f64) -> f64 {
        libm::exp(x)
    }

    fn ln(x: f64) -> f64 {
        libm::log(x)
    }

    fn sin(x: f64) -> f64 {
        libm::sin(x)
    }

    fn cos(x: f64) -> f64 {
        libm::cos(x)
    }

    fn atan2(y: f64, x: f64) -> f64 {
        libm::atan2(y, x)
    }

    fn mul_add(x: f64, a: f64, b: f64) -> f64 {
        libm::fma(x, a, b)
    }

    fn powi(x: f64, n: i64) -> f64 {
        libm::pow(x, n as f64)
    }

    fn hypot(x: f64, y: f64) -> f64 {
        libm::hypot(x, y)
    }

    fn ln_1p(x: f64) -> f64 {
        libm::log1p(x)
    }

    fn exp_m1(x: f64) -> f64 {
        libm::expm1(x)
    }

    fn abs(x: f64) -> f64 {
        libm::fabs(x)
    }

    fn floor(x: f64) -> f64 {
        libm::floor(x)
    }

    fn ceil(x: f64) -> f64 {
        libm::ceil(x)
    }

    fn round(x: f64) -> f64 {
        libm::round(x)
    }
}

/// Registers [`Libm`]. In a child module because the generated definitions
/// can't share a namespace with the `extern` declarations above.
#[cfg(feature = "libm")]
mod libm_backend {
    set_float_backend!(super::Libm);
}

// The tests compare against `libm`, so they need the feature.
#[cfg(all(test, feature = "libm"))]
mod tests {
    use super::{FloatBackend, copysign, trunc};

    /// Only the required methods, so the tests exercise the defaults.
    struct Defaults;

    impl FloatBackend for Defaults {
        fn powf(_: f64, _: f64) -> f64 {
            unimplemented!()
        }

        fn sqrt(_: f64) -> f64 {
            unimplemented!()
        }

        fn cbrt(_: f64) -> f64 {
            unimplemented!()
        }

        fn exp(_: f64) -> f64 {
            unimplemented!()
        }

        fn ln(_: f64) -> f64 {
            unimplemented!()
        }

        fn sin(_: f64) -> f64 {
            unimplemented!()
        }

        fn cos(_: f64) -> f64 {
            unimplemented!()
        }

        fn atan2(_: f64, _: f64) -> f64 {
            unimplemented!()
        }
    }

    fn same(a: f64, b: f64) -> bool {
        a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
    }

    /// The `core`-only defaults must match `libm` exactly, sign of zero
    /// included.
    #[test]
    fn test_rounding_defaults() {
        let inputs = [
            0.0,
            -0.0,
            0.3,
            -0.3,
            0.5,
            -0.5,
            1.5,
            -1.5,
            2.5,
            -2.5,
            99.999,
            -99.999,
            4_503_599_627_370_495.5,
            4_503_599_627_370_496.0,
            -4_503_599_627_370_497.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
            f64::MIN_POSITIVE,
            // Largest double below 0.5: `floor(x + 0.5)` would wrongly give 1.
            0.499_999_999_999_999_94,
            -0.499_999_999_999_999_94,
        ];

        for x in inputs {
            assert!(same(Defaults::abs(x), libm::fabs(x)), "abs({x})");
            assert!(same(Defaults::floor(x), libm::floor(x)), "floor({x})");
            assert!(same(Defaults::ceil(x), libm::ceil(x)), "ceil({x})");
            assert!(same(Defaults::round(x), libm::round(x)), "round({x})");
            assert!(same(trunc(x), libm::trunc(x)), "trunc({x})");
        }

        assert!(same(copysign(1.0, -0.0), -1.0));
    }
}
