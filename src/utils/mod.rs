pub mod math;
#[cfg(not(feature = "std"))] pub mod no_std;
pub mod random;

pub trait FromRef<T> {
    fn from_ref(value: &T) -> Self;
}
