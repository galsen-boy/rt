use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Cube {
        Cube { min, max, material }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t1 = (self.min.x() - r.origin.x()) / r.direction.x();
        let t2 = (self.max.x() - r.origin.x()) / r.direction.x();

        let t3 = (self.min.y() - r.origin.y()) / r.direction.y();
        let t4 = (self.max.y() - r.origin.y()) / r.direction.y();

        let t5 = (self.min.z() - r.origin.z()) / r.direction.z();
        let t6 = (self.max.z() - r.origin.z()) / r.direction.z();

        let t_min = t_min.max(t1.min(t2)).max(t3.min(t4)).max(t5.min(t6));
        let t_max = t_max.min(t1.max(t2)).min(t3.max(t4)).min(t5.max(t6));

        if t_max > t_min {
            let t = t_min;
            let p = r.at(t);

            let normal = if p.x() < self.min.x() + 0.00001 {
                Vec3::new(-1.0, 0.0, 0.0)
            } else if p.x() > self.max.x() - 0.00001 {
                Vec3::new(1.0, 0.0, 0.0)
            } else if p.y() < self.min.y() + 0.00001 {
                Vec3::new(0.0, -1.0, 0.0)
            } else if p.y() > self.max.y() - 0.00001 {
                Vec3::new(0.0, 1.0, 0.0)
            } else if p.z() < self.min.z() + 0.00001 {
                Vec3::new(0.0, 0.0, -1.0)
            } else if p.z() > self.max.z() - 0.00001 {
                Vec3::new(0.0, 0.0, 1.0)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            };

            Some(HitRecord {
                t,
                point: p,
                u: 1.0,
                v: 1.0,
                normal,
                material: self.material,
            })
        } else {
            None
        }
    }
}
