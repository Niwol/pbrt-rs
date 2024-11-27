#[cfg(feature = "float_as_double")]
pub type Float = f64;
#[cfg(not(feature = "float_as_double"))]
pub type Float = f32;
