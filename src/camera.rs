use crate::ray::Ray;//Importation de la structure Ray définie dans le module ray
use crate::vec3::Vec3;//Importation de la structure Vec3 définie dans le module vec3.
/*Importation de tout le contenu de la pré-configuration de rand,
 un crate pour la génération de nombres aléatoires.*/
use rand::prelude::*;
/*Importation des traits Deserialize et Serialize de serde, 
un crate pour la sérialisation et la désérialisation.*/
use serde::{Deserialize, Serialize};
/*La structure Camera représente une caméra dans un système de rendu graphique. Les champs sont :*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    origin: Vec3,//Le point d'origine de la caméra.
    lower_left_corner: Vec3,//Le coin inférieur gauche du plan de l'image.
    horizontal: Vec3,//Le vecteur horizontal du plan de l'image.
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,

    height: u32,
    width: u32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aperture: f64,
        height: u32,
        width: u32
    ) -> Camera {
        let aspect = height as f64 / width as f64;

        let lens_radius = aperture / 2.0;
        let focus_dist = (look_from - look_at).length();
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        Camera {
            lower_left_corner: origin
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin,
            lens_radius,
            u,
            v,
            height,
            width,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
        if Vec3::dot(&p, &p) < 1.0 {
            return p;
        }
    }
}
