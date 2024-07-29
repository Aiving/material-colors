pub mod math;
#[cfg(not(feature = "std"))]
pub(crate) mod no_std;
pub mod random;
