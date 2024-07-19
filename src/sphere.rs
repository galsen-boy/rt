use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    point: r.at(temp),
                    normal: (r.at(temp) - self.center) / self.radius,
                    u: 0.0,
                    v: 0.0,
                    material: self.material,
                });
            }

            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    point: r.at(temp),
                    normal: (r.at(temp) - self.center) / self.radius,
                    u: 0.0,
                    v: 0.0,
                    material: self.material,
                });
            }
        }

        None
    }
}
