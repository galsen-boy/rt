use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Plane {
    normal: Vec3,
    dist: f64,
    width: f64,
    height: f64,
    material: Material,
}

impl Plane {
    pub fn new(normal: Vec3, dist: f64, width: f64, height: f64, material: Material) -> Plane {
        Plane {
            normal: normal * -1.0,
            dist,
            width,
            height,
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = Vec3::dot(&self.normal, &r.direction);
        if denom > 1e-6 {
            let t = (-self.dist - Vec3::dot(&self.normal, &r.origin)) / denom;

            if t > t_min && t < t_max {
                let point = r.at(t);
                let d = point.x().abs().max(point.y().abs().max(point.z().abs()));

                if d < self.width / 2.0 && d < self.height / 2.0 {
                    return Some(HitRecord {
                        t,
                        point,
                        normal: self.normal,
                        u: 0.0,
                        v: 0.0,
                        material: self.material,
                    });
                }
            }
        }

        None
    }
}
