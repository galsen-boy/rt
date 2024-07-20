use crate::Vec3;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Default, Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[test]
fn test_ray() {
    let p = Vec3::new(1.1, 1.2, 1.3);
    let q = Vec3::new(2.2, 2.3, 2.4);

    let r = Ray::new(p, q);

    assert_approx_eq!(r.origin.x(), 1.1);
    assert_approx_eq!(r.origin.y(), 1.2);
    assert_approx_eq!(r.origin.z(), 1.3);
    assert_approx_eq!(r.direction.x(), 2.2);
    assert_approx_eq!(r.direction.y(), 2.3);
    assert_approx_eq!(r.direction.z(), 2.4);
}

#[test]
fn test_ray_at() {
    let p1 = Vec3::new(0.0, 0.0, 0.0);
    let p2 = Vec3::new(1.0, 2.0, 3.0);

    let ray = Ray::new(p1, p2);
    let s = ray.at(0.5);

    assert_approx_eq!(s.x(), 0.5);
    assert_approx_eq!(s.y(), 1.0);
    assert_approx_eq!(s.z(), 1.5);
}
