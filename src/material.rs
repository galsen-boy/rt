use crate::color::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
    Dielectric { ref_idx: f64 },
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Color::default(),
        }
    }
}

pub fn scatter(material: &Material, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    match material {
        Material::Lambertian { albedo } => {
            let target = rec.point + rec.normal + random_in_unit_sphere();

            Some((*albedo, Ray::new(rec.point, target - rec.point)))
        }
        Material::Metal { albedo } => {
            let reflected = reflect(&Vec3::unit_vector(&ray_in.direction), &rec.normal);

            if Vec3::dot(&Ray::default().direction, &rec.normal) > 0.0 {
                Some((*albedo, Ray::new(rec.point, reflected)))
            } else {
                None
            }
        }
        Material::Dielectric { ref_idx } => {
            let outward_normal: Vec3;
            let reflected = reflect(&ray_in.direction, &rec.normal);
            let ni_over_nt: f64;

            let cosine: f64 = if Vec3::dot(&ray_in.direction, &rec.normal) > 0.0 {
                outward_normal = -rec.normal;
                ni_over_nt = *ref_idx;

                ref_idx * Vec3::dot(&ray_in.direction, &rec.normal) / ray_in.direction.length()
            } else {
                outward_normal = rec.normal;
                ni_over_nt = 1.0 / ref_idx;

                -Vec3::dot(&ray_in.direction, &rec.normal) / ray_in.direction.length()
            };

            let refracted: (Vec3, f64) =
                match refract(&ray_in.direction, &outward_normal, ni_over_nt) {
                    Some(v) => (v, schlick(cosine, *ref_idx)),
                    None => (Vec3::default(), 1.0),
                };

            let mut rng = rand::thread_rng();

            Some((
                Color::new(1.0, 1.0, 1.0),
                if rng.gen::<f64>() < refracted.1 {
                    Ray::new(rec.point, reflected)
                } else {
                    Ray::new(rec.point, refracted.0)
                },
            ))
        }
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * *n
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}
