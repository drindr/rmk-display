#![macro_use]
#[cfg(feature = "defmt")]
#[collapse_debuginfo(yes)]
macro_rules! unwrap {
    ($($x:tt)*) => {
        ::defmt::unwrap!($($x)*)
    };
}
