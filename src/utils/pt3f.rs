use crate::types::*;
use crate::utils::f32;

pub fn min(p1: &Point3f, p2: &Point3f) -> Point3f {
    Point3f::new(
        f32::min(p1.x, p2.x),
        f32::min(p1.y, p2.y),
        f32::min(p1.z, p2.z),
    )
}

pub fn max(p1: &Point3f, p2: &Point3f) -> Point3f {
    Point3f::new(
        f32::max(p1.x, p2.x),
        f32::max(p1.y, p2.y),
        f32::max(p1.z, p2.z),
    )
}

pub fn almost_eq(p1: &Point3f, p2: &Point3f) -> bool {
    f32::almost_eq(p1.x, p2.x) && f32::almost_eq(p1.y, p2.y) && f32::almost_eq(p1.z, p2.z)
}

pub fn lerp(a: &Point3f, b: &Point3f, t: f32) -> Point3f {
    Point3f::new(
        f32::lerp(a.x, b.x, t),
        f32::lerp(a.y, b.y, t),
        f32::lerp(a.z, b.z, t),
    )
}

pub fn clerp(a: &Point3f, b: &Point3f, t: f32) -> Point3f {
    Point3f::new(
        f32::clerp(a.x, b.x, t),
        f32::clerp(a.y, b.y, t),
        f32::clerp(a.z, b.z, t),
    )
}
