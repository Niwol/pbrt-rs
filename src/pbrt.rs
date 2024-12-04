use crate::options::PBRTOptions;

#[cfg(feature = "float_as_double")]
pub type Float = f64;
#[cfg(not(feature = "float_as_double"))]
pub type Float = f32;

pub struct PBRT {}

impl PBRT {
    pub fn new(_options: &mut PBRTOptions) -> Self {
        Self {}
    }

    pub fn render_cpu(&self) {}
}
