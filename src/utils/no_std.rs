#[allow(dead_code)]
pub trait FloatExt {
    #[must_use]
    fn abs(self) -> Self;

    #[must_use]
    fn mul_add(self, x: Self, y: Self) -> Self;

    #[must_use]
    fn powf(self, n: Self) -> Self;

    #[must_use]
    fn powi(self, n: i64) -> Self;

    #[must_use]
    fn cos(self) -> Self;

    #[must_use]
    fn sin(self) -> Self;

    #[must_use]
    fn cbrt(self) -> Self;

    #[must_use]
    fn ln(self) -> Self;

    #[must_use]
    fn ln_1p(self) -> Self;

    #[must_use]
    fn exp(self) -> Self;

    #[must_use]
    fn exp_m1(self) -> Self;

    #[must_use]
    fn round(self) -> Self;

    #[must_use]
    fn ceil(self) -> Self;

    #[must_use]
    fn floor(self) -> Self;

    #[must_use]
    fn sqrt(self) -> Self;

    #[must_use]
    fn hypot(self, n: Self) -> Self;

    #[must_use]
    fn atan2(self, n: Self) -> Self;
}

impl FloatExt for f64 {
    fn abs(self) -> Self {
        libm::fabs(self)
    }

    fn mul_add(self, x: Self, y: Self) -> Self {
        libm::fma(self, x, y)
    }

    fn powf(self, n: Self) -> Self {
        libm::pow(self, n)
    }

    fn powi(self, n: i64) -> Self {
        libm::pow(self, n as Self)
    }

    fn cos(self) -> Self {
        libm::cos(self)
    }

    fn sin(self) -> Self {
        libm::sin(self)
    }

    fn cbrt(self) -> Self {
        libm::cbrt(self)
    }

    fn ln(self) -> Self {
        libm::log(self)
    }

    fn ln_1p(self) -> Self {
        libm::log1p(self)
    }

    fn exp(self) -> Self {
        libm::exp(self)
    }

    fn exp_m1(self) -> Self {
        libm::expm1(self)
    }

    fn round(self) -> Self {
        libm::round(self)
    }

    fn ceil(self) -> Self {
        libm::ceil(self)
    }

    fn floor(self) -> Self {
        libm::floor(self)
    }

    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }

    fn hypot(self, n: Self) -> Self {
        libm::hypot(self, n)
    }

    fn atan2(self, n: Self) -> Self {
        libm::atan2(self, n)
    }
}
