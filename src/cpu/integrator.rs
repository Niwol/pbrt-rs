use crate::pbrt::Float;

pub trait Integrator {
    fn to_string(&self) -> String;
    fn tr(&self /*, p0: &Interaction, p1: &Interaction, lambda: SampledWavelength */)
    /* -> SampledSpectrum */;
}

pub trait Renderer {
    fn render(&self);
}

pub trait Intersector {
    fn intersect(&self /*, ray: &Ray */, t_max: Float) /* -> Option<ShapeIntersection> */;
    fn intersect_p(&self /*, ray: &Ray */, t_max: Float) -> bool;
    fn unoccluded(&self /*, p0: Interaction, p1: Interaction */) -> bool {
        false
    }
}
