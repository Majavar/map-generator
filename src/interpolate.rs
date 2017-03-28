use config::Interpolation;

pub trait Lerp {
    fn lerp(v0: f64, v1: f64, f64: f64) -> f64;

    fn linear(v0: f64, v1: f64, f64: f64) -> f64;
    fn cubic(v0: f64, v1: f64, f64: f64) -> f64;
    fn quintic(v0: f64, v1: f64, f64: f64) -> f64;
    fn cosine(v0: f64, v1: f64, f64: f64) -> f64;
}


impl Lerp for f64 {
    fn lerp(v0: f64, v1: f64, t: f64) -> f64 {
        v0 * (1.0 - t) + v1 * t
    }

    fn linear(v0: f64, v1: f64, t: f64) -> f64 {
        let v = t;
        Self::lerp(v0, v1, v)
    }
    fn cubic(v0: f64, v1: f64, t: f64) -> f64 {
        let v = t * t * (3.0 - t * 2.0);
        Self::lerp(v0, v1, v)
    }
    fn quintic(v0: f64, v1: f64, t: f64) -> f64 {
        let v = t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
        Self::lerp(v0, v1, v)
    }
    fn cosine(v0: f64, v1: f64, t: f64) -> f64 {
        let v = (1.0 - (::std::f64::consts::PI * t).cos()) * 0.5;
        Self::lerp(v0, v1, v)
    }
}

pub fn get(interpolation: Interpolation) -> fn(f64, f64, f64) -> f64 {
    match interpolation {
        Interpolation::Linear => f64::linear,
        Interpolation::Cubic => f64::cubic,
        Interpolation::Quintic => f64::quintic,
        Interpolation::Cosine => f64::cosine,
    }
}
