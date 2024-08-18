pub mod math;
#[cfg(all(not(feature = "std"), feature = "libm"))]
pub mod no_std;
pub mod random;

pub trait FromRef<T> {
    fn from_ref(value: &T) -> Self;
}
