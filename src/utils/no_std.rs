#[allow(dead_code)]
pub trait FloatExt {
    fn abs(self) -> Self;
    fn mul_add(self, x: Self, y: Self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn powi(self, n: i64) -> Self;
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn cbrt(self) -> Self;
    fn ln(self) -> Self;
    fn ln_1p(self) -> Self;
    fn exp(self) -> Self;
    fn exp_m1(self) -> Self;
    fn round(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn sqrt(self) -> Self;
    fn hypot(self, n: Self) -> Self;
    fn atan2(self, n: Self) -> Self;
}

#[cfg(not(feature = "no-libm"))]
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
