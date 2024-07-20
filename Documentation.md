# camera.rs
## Importations et Dépendances

Le code commence par importer plusieurs modules et crates nécessaires :
```
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
```
-``crate::ray::Ray`` : Importation de la structure Ray définie dans le module ray.
-``crate::vec3::Vec3 ``: Importation de la structure Vec3 définie dans le module vec3.
-``rand::prelude::* ``: Importation de tout le contenu de la pré-configuration de rand, un crate pour la génération de nombres aléatoires.
-``serde::{Deserialize, Serialize}`` : Importation des traits Deserialize et Serialize de serde, un crate pour la sérialisation et la désérialisation.

**Définition de la Structure Camera**
```
#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    height: u32,
    width: u32,
}
```
La structure Camera représente une caméra dans un système de rendu graphique. Les champs sont :

- ``origin`` : Le point d'origine de la caméra.
- ``lower_left_corner`` : Le coin inférieur gauche du plan de l'image.
- ``horizontal`` : Le vecteur horizontal du plan de l'image.
- ``vertical`` : Le vecteur vertical du plan de l'image.
- ``lens_radius`` : Le rayon de la lentille de la caméra (utilisé pour la profondeur de champ).
- ``u, v`` : Vecteurs orthogonaux utilisés pour calculer les offsets des rayons.
- ``height, width`` : La hauteur et la largeur de l'image.

## Implémentation de la Structure Camera
Méthode ``new``
```
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
```
La méthode new crée une nouvelle instance de Camera. Voici un aperçu des étapes :

 1. Calcul de l'aspect ratio :

```
let aspect = height as f64 / width as f64;
```
2. Calcul du rayon de la lentille et de la distance focale :
```
let lens_radius = aperture / 2.0;
let focus_dist = (look_from - look_at).length();
```
3. Conversion de l'angle de vue vertical (vfov) de degrés en radians et calcul des demi-hauteur et demi-largeur :
```
let theta = vfov * std::f64::consts::PI / 180.0;
let half_height = (theta / 2.0).tan();
let half_width = aspect * half_height;
```
4. Définition de l'origine et calcul des vecteurs de base de la caméra :
```
let origin = look_from;
let w = Vec3::unit_vector(&(look_from - look_at));
let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
let v = Vec3::cross(&w, &u);
```
5. Calcul des coins et des dimensions du plan de l'image :
```
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
```
Méthode ``get_ray``
```
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
```
La méthode ``get_ray`` génère un rayon partant de la caméra vers un point de l'image :

1. Calcul d'un point aléatoire sur la lentille :
```
let rd = self.lens_radius * random_in_unit_disk();
```
2. Calcul de l'offset du rayon basé sur la lentille :
```
let offset = self.u * rd.x() + self.v * rd.y();
```
3. Création d'un rayon avec origine et direction calculées :
```
    Ray {
        origin: self.origin + offset,
        direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
            - self.origin
            - offset,
    }
```
**Fonction random_in_unit_disk**
```
pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
        if Vec3::dot(&p, &p) < 1.0 {
            return p;
        }
    }
}
```
Cette fonction génère un point aléatoire à l'intérieur d'un disque unité :

1. Initialisation d'un générateur de nombres aléatoires :
```
let mut rng = rand::thread_rng();
```
2. Boucle jusqu'à trouver un point à l'intérieur du disque unité :
```
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
        if Vec3::dot(&p, &p) < 1.0 {
            return p;
        }
    }
```
En somme, ce code définit une caméra pour un ray tracer avec des méthodes pour initialiser la caméra et générer des rayons à partir de celle-ci. La fonction random_in_unit_disk est utilisée pour créer un flou de profondeur de champ en perturbant légèrement les rayons en fonction de la lentille de la caméra.
# colors.rs
Ce code définit une nouvelle Color en utilisant le type Vec3 et inclut un ensemble de tests unitaires pour vérifier diverses opérations sur ce type. Voici une explication détaillée de ce code :
**Définition de ``Color``**
```
use crate::vec3::Vec3;
pub type Color = Vec3;
```
``Color`` est défini comme un alias pour Vec3. Cela signifie que Color et Vec3 sont interchangeables dans le code. Vec3 est probablement une structure qui représente un vecteur à trois dimensions, souvent utilisé pour représenter des couleurs RGB (Rouge, Vert, Bleu).
**Module de Tests**
Les tests unitaires sont définis dans un module de tests en utilisant l'attribut #[cfg(test)] pour s'assurer qu'ils ne sont compilés et exécutés que lors des tests.
```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(1.0, 0.5, 0.25);
        assert_eq!(color.r(), 1.0);
        assert_eq!(color.g(), 0.5);
        assert_eq!(color.b(), 0.25);
    }
    // ... (autres tests)
}
```
**Explication des Tests**

1. Test de Création de Color
```
#[test]
fn test_color_new() {
    let color = Color::new(1.0, 0.5, 0.25);
    assert_eq!(color.r(), 1.0);
    assert_eq!(color.g(), 0.5);
    assert_eq!(color.b(), 0.25);
}
```
Ce test vérifie que la création d'un objet Color avec des valeurs spécifiques fonctionne correctement et que les méthodes d'accès pour chaque composant (``r``,`` g``, ``b``) retournent les valeurs correctes.

2. Test de l'Addition
```
#[test]
fn test_add() {
    let color1 = Color::new(1.0, 0.5, 0.25);
    let color2 = Color::new(0.5, 0.25, 0.125);
    let result = color1 + color2;
    assert_eq!(result.r(), 1.5);
    assert_eq!(result.g(), 0.75);
    assert_eq!(result.b(), 0.375);
}
```
Ce test vérifie que l'addition de deux objets Color fonctionne comme prévu, en additionnant chaque composant correspondant.

3. Test de la Soustraction
```
#[test]
fn test_sub() {
    let color1 = Color::new(1.0, 0.5, 0.25);
    let color2 = Color::new(0.5, 0.25, 0.125);
    let result = color1 - color2;
    assert_eq!(result.r(), 0.5);
    assert_eq!(result.g(), 0.25);
    assert_eq!(result.b(), 0.125);
}
```
Ce test vérifie que la soustraction de deux objets Color fonctionne correctement.

4. Test de la Négation
```
#[test]
fn test_neg() {
    let color = Color::new(1.0, 0.5, 0.25);
    let result = -color;
    assert_eq!(result.r(), -1.0);
    assert_eq!(result.g(), -0.5);
    assert_eq!(result.b(), -0.25);
}
```
Ce test vérifie que la négation d'un objet Color fonctionne correctement, en inversant le signe de chaque composant.

5. Test de la Multiplication de Couleurs
```
#[test]
fn test_mul_colors() {
    let color1 = Color::new(1.0, 0.5, 0.25);
    let color2 = Color::new(0.5, 0.25, 4.0);
    let result = color1 * color2;
    assert_eq!(result.r(), 0.5);
    assert_eq!(result.g(), 0.125);
    assert_eq!(result.b(), 1.0);
}
```
Ce test vérifie que la multiplication de deux objets Color fonctionne correctement, en multipliant chaque composant correspondant.

6. Test de la Multiplication par un Scalaire
```
#[test]
fn test_mul_color() {
    let color = Color::new(1.0, 0.5, 0.25);
    let result = color * 2.0;
    assert_eq!(result.r(), 2.0);
    assert_eq!(result.g(), 1.0);
    assert_eq!(result.b(), 0.5);
}
```
Ce test vérifie que la multiplication d'un objet Color par un scalaire fonctionne correctement.

7.Test de la Division par un Scalaire
```
    #[test]
    fn test_div_color() {
        let color = Color::new(1.0, 0.5, 0.25);
        let result = color / 2.0;
        assert_eq!(result.r(), 0.5);
        assert_eq!(result.g(), 0.25);
        assert_eq!(result.b(), 0.125);
    }
```

    Ce test vérifie que la division d'un objet Color par un scalaire fonctionne correctement.

**Conclusion**
Ces tests couvrent les opérations de base sur les objets Color en vérifiant la création, l'addition, la soustraction, la négation, la multiplication (avec d'autres couleurs et des scalaires) et la division par un scalaire. Ces opérations sont cruciales pour les manipulations de couleurs dans un système de rendu graphique.
# config.rs
Ce code montre comment sérialiser et désérialiser des données non traitées représentant des objets géométriques et des caméras dans une application graphique. Chaque type d'objet a une structure "non traitée" qui peut être convertie en une forme traitée utilisée pour le rendu. Voici une explication détaillée du code :
**Importations**
```
use crate::{
    camera::Camera,
    cube::Cube,
    hit::{Hittable, HittableList},
    material::Material,
    plane_surf::Plane,
    sphere::Sphere,
    vec3::Vec3,
    cylinder::Cylinder,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
```
Les modules et les traits utilisés dans ce fichier sont importés ici. serde est utilisé pour la sérialisation et la désérialisation.
## Trait ``UnprocessedData``
```
#[typetag::serde]
pub trait UnprocessedData: Debug {
    fn process(&self) -> Box<dyn Hittable>;
}
```
``UnprocessedData`` est un trait qui oblige les implémentations à fournir une méthode process qui retourne un objet Hittable (quelque chose qui peut être frappé par un rayon).
## Structures Non Traitée et leurs Implémentations

   1. UnprocessedCube
```
#[derive(Debug, Serialize, Deserialize)]
pub struct UnprocessedCube {
    p0: Vec3,
    p1: Vec3,
    mat: Material,
}

#[typetag::serde(name = "Cube")]
impl UnprocessedData for UnprocessedCube {
    fn process(&self) -> Box<dyn Hittable> {
        Box::new(Cube::new(self.p0, self.p1, self.mat))
    }
}
```
Cette structure représente un cube non traité. Elle est sérialisable et désérialisable grâce à serde. La méthode process convertit cette structure en un Cube utilisable.

2. UnprocessedPlane
```
#[derive(Debug, Serialize, Deserialize)]
pub struct UnprocessedPlane {
    normal: Vec3,
    dist: f64,
    width: f64,
    height: f64,
    material: Material,
}

#[typetag::serde(name = "Plane")]
impl UnprocessedData for UnprocessedPlane {
    fn process(&self) -> Box<dyn Hittable> {
        Box::new(Plane::new(
            self.normal,
            self.dist,
            self.width,
            self.height,
            self.material,
        ))
    }
}
```
Cette structure représente un plan non traité. La méthode process convertit cette structure en un Plane utilisable.

3. UnprocessedSphere
```
#[derive(Debug, Serialize, Deserialize)]
pub struct UnprocessedSphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

#[typetag::serde(name = "Sphere")]
impl UnprocessedData for UnprocessedSphere {
    fn process(&self) -> Box<dyn Hittable> {
        Box::new(Sphere::new(self.center, self.radius, self.material))
    }
}
```
Cette structure représente une sphère non traitée. La méthode process convertit cette structure en un Sphere utilisable.

4. UnprocessedCylinder
```
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UnprocessedCylinder {
        center: Vec3,
        radius: f64,
        height: f64,
        material: Material,
    }

    #[typetag::serde(name = "Cylinder")]
    impl UnprocessedData for UnprocessedCylinder {
        fn process(&self) -> Box<dyn Hittable> {
            Box::new(Cylinder::new(self.center, self.radius, self.height, self.material))
        }
    }
```
    Cette structure représente un cylindre non traité. La méthode process convertit cette structure en un Cylinder utilisable.

## UnprocessedCamera
```
#[derive(Debug, Serialize, Deserialize)]
pub struct UnprocessedCamera {
    look_from: Vec3,
    look_at: Vec3,
    vup: Vec3,
    vfov: f64,
    aperture: f64,
}

impl UnprocessedCamera {
    fn process(&self, height: u32, width: u32) -> Camera {
        Camera::new(
            self.look_from,
            self.look_at,
            self.vup,
            self.vfov,
            self.aperture,
            height,
            width,
        )
    }
}
```
Cette structure représente une caméra non traitée. La méthode process convertit cette structure en une Camera utilisable, prenant en compte la hauteur et la largeur de l'image.
## Config
```
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(alias = "world")]
    unprocessed_data: Vec<Box<dyn UnprocessedData>>,
    #[serde(alias = "camera")]
    cam: UnprocessedCamera,
    #[serde(alias = "light")]
    light: i32,
    #[serde(alias = "samples")]
    samples: i32,
    #[serde(alias = "width")]
    width: u32,
    #[serde(alias = "height")]
    height: u32,
}

impl Config {
    pub fn process(self) -> Application {
        Application {
            world: HittableList::new(self.unprocessed_data.iter().map(|d| d.process()).collect()),
            camera: self.cam.process(self.width, self.height),
            light: self.light,
            samples: self.samples,
            width: self.width,
            height: self.height,
        }
    }
}
```
``Config`` est une structure qui contient toutes les données non traitées nécessaires pour configurer une scène. La méthode process convertit ces données non traitées en une application utilisable.
## Application
```
#[derive(Debug)]
pub struct Application {
    pub world: HittableList,
    pub camera: Camera,
    pub light: i32,
    pub samples: i32,
    pub height: u32,
    pub width: u32,
}
```
Application est la structure finale contenant les objets traités (world), la caméra (camera), et les paramètres de rendu (light, samples, height, width).
# cube.rs
Ce code montre l'implémentation d'un cube et la manière dont il peut être détecté par un rayon dans un moteur de rendu 3D. Voici une explication détaillée du code :
## Importations
```
use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
```
Ces importations sont nécessaires pour utiliser les structures et les traits définis dans d'autres modules du projet, comme ``Ray``, ``Vec3``, ``HitRecord``, ``Hittable``, et ``Material``.
## Structure ``Cube``
```
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
```
La structure ``Cube`` représente un cube avec deux points (min et max) qui définissent ses coins opposés dans l'espace 3D, ainsi qu'un matériau (material).
## Implémentation du Trait ``Hittable`` pour ``Cube``
```
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
```
## Explication de la Méthode ``hit``

1. Calcul des valeurs t pour chaque axe :
    - Les valeurs t1 et t2 sont calculées pour l'axe x.
    - Les valeurs t3 et t4 sont calculées pour l'axe y.
    - Les valeurs t5 et t6 sont calculées pour l'axe z.

2. Calcul de t_min et t_max :
    - ``t_min`` est le maximum des minimums des intervalles de t pour chaque axe.
    - ``t_max`` est le minimum des maximums des intervalles de t pour chaque axe.
    - Ces valeurs déterminent si le rayon intersecte le cube dans tous les axes.

3. Détection de l'intersection :
       - Si ``t_max`` est supérieur à ``t_min``, cela signifie que le rayon intersecte le cube.
        Le point d'intersection ``p`` est calculé en utilisant ``r.at(t)``.

4. Calcul de la normale :
    - La normale au point d'intersection est déterminée en vérifiant la proximité de p aux bords du cube.

5. Création et retour d'un HitRecord :
    - Si une intersection est détectée, un HitRecord est créé avec le temps d'intersection ``t``, le point d'intersection ``p``, les coordonnées de texture (``u`` et ``v``), la normale, et le matériau.
    - Si aucune intersection n'est détectée, None est retourné.

Ce code permet de détecter si un rayon intersecte un cube dans un espace 3D et de retourner les informations nécessaires pour le rendu de l'intersection, comme la position, la normale et le matériau du point d'intersection.
## cylinder.rs
Ce code montre l'implémentation d'un cylindre et la manière dont il peut être détecté par un rayon dans un moteur de rendu 3D. Voici une explication détaillée du code :
## Importations
```
use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
```
Ces importations sont nécessaires pour utiliser les structures et les traits définis dans d'autres modules du projet, comme Ray, Vec3, HitRecord, Hittable, et Material.
## Structure ``Cylinder``
```
#[derive(Debug)]
pub struct Cylinder {
    center: Vec3,
    radius: f64,
    height: f64,
    material: Material,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f64, height: f64, material: Material) -> Cylinder {
        Cylinder {
            center,
            radius,
            height,
            material,
        }
    }
}
```
La structure ``Cylinder`` représente un cylindre avec un centre (``center``), un rayon (``radius``), une hauteur (``height``) et un matériau (``material``).
## Implémentation du Trait ``Hittable`` pour ``Cylinder``
```
impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;

        let a = r.direction.dot_xz(&r.direction);
        let b = oc.dot_xz(&r.direction);
        let c = oc.dot_xz(&oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        let mut t_array: [f64; 4] = [0.0; 4];
        let mut valid_array: [bool; 4] = [false; 4];
        let mut poi_array: [Option<HitRecord>; 4] = [None; 4];

        t_array[2] = (self.center.y() - r.origin.y()) / r.direction.y();
        t_array[3] = (self.center.y() + self.height - r.origin.y()) / r.direction.y();

        let mut poi = oc + t_array[2] * r.direction;
        let mut normal = (r.at(t_array[2]) - self.center) / self.radius;

        if t_array[2] > t_min && t_array[2] < t_max && poi.dot_xz(&poi) < self.radius * self.radius { 
            valid_array[2] = true;
            poi_array[2] = Some(HitRecord {
                t: t_array[2],
                point: r.at(t_array[2]),
                normal,
                u: 0.0,
                v: 0.0,
                material: self.material.clone(),
            });
        } else {
            t_array[2] = 100e6;
        }

        poi = oc + t_array[3] * r.direction;
        normal = (r.at(t_array[3]) - self.center) / self.radius;

        if t_array[3] > t_min && t_array[3] < t_max && poi.dot_xz(&poi) < self.radius * self.radius { 
            valid_array[3] = true;
            poi_array[3] = Some(HitRecord {
                t: t_array[3],
                point: r.at(t_array[3]),
                normal,
                u: 0.0,
                v: 0.0,
                material: self.material.clone(),
            });
        } else {
            t_array[3] = 100e6;
        }

        if discriminant > 0.0 {
            t_array[0] = (-b - discriminant.sqrt()) / a;
            let mut y = r.origin.y() + t_array[0] * r.direction.y();

            if t_array[0] < t_max && t_array[0] > t_min && y >= self.center.y() - 0.0001 && y <= self.center.y() + self.height + 0.0001 {
                valid_array[0] = true;
                poi_array[0] = Some(HitRecord {
                    t: t_array[0],
                    point: r.at(t_array[0]),
                    normal: (r.at(t_array[0]) - self.center) / self.radius,
                    u: 0.0,
                    v: 0.0,
                    material: self.material.clone(),
                });
            } else {
                t_array[0] = 100e6;
            }

            t_array[1] = (-b + discriminant.sqrt()) / a;
            y = r.origin.y() + t_array[1] * r.direction.y();

            if t_array[1] < t_max && t_array[1] > t_min && y >= self.center.y() - 0.0001 && y <= self.center.y() + self.height + 0.0001 {
                valid_array[1] = true;
                poi_array[1] = Some(HitRecord {
                    t: t_array[1],
                    point: r.at(t_array[1]),
                    normal: (r.at(t_array[1]) - self.center) / self.radius,
                    u: 0.0,
                    v: 0.0,
                    material: self.material.clone(),
                });
            } else {
                t_array[1] = 100e6;
            }
        }

        if !valid_array[0] && !valid_array[1] && !valid_array[2] && !valid_array[3] {
            return None;
        }

        let mut min_index = 0;
        let mut min_value = 10e6;
        for (i, value) in t_array.iter().enumerate() {
            if value < &min_value {
                min_value = *value;
                min_index = i;
            }
        }

        let valid_poi = poi_array[min_index];
        valid_poi
    }
}
```
## Explication de la Méthode ``hit``

1. Calcul des valeurs de t pour chaque axe :
    - ``t_array[2]`` et ``t_array[3]`` sont calculés pour les intersections avec les plans supérieur et inférieur du cylindre.
    - Les coefficients ``a``, ``b``, et ``c`` sont calculés pour l'équation quadratique décrivant l'intersection avec les côtés du cylindre.
    - Le discriminant est calculé pour déterminer s'il y a des intersections avec les côtés du cylindre.

2. Détection des intersections avec les plans :
    - Pour chaque plan (supérieur et inférieur), on calcule le point d'intersection et vérifie si le rayon intersecte le cylindre à cet endroit.
    - Si une intersection est détectée, un HitRecord est créé avec les informations d'intersection.

3. Détection des intersections avec les côtés :
    - Si le discriminant est positif, les valeurs t pour les intersections avec les côtés du cylindre sont calculées.
    - On vérifie si ces intersections sont dans les limites de la hauteur du cylindre.
    - Si une intersection est détectée, un HitRecord est créé avec les informations d'intersection.

4. Sélection de l'intersection valide la plus proche :
    - Si aucune intersection valide n'est trouvée, la méthode retourne None.
    - Sinon, on sélectionne l'intersection la plus proche et retourne le HitRecord correspondant.

Ce code permet de détecter si un rayon intersecte un cylindre dans un espace 3D et de retourner les informations nécessaires pour le rendu de l'intersection, comme la position, la normale et le matériau du point d'intersection.
# flags.rs
Ce code présente un outil en ligne de commande qui utilise clap pour analyser les arguments de ligne de commande et ron pour la configuration. Il lit un fichier de configuration, le parse et crée une instance d'application basée sur cette configuration. Voici une explication détaillée :
## Imports
```
use std::path::PathBuf;
use clap::Parser;
use crate::config::{Application, Config};
```
- ``PathBuf`` est utilisé pour stocker le chemin du fichier de configuration.
- ``Parser`` est importé de clap pour définir et analyser les arguments de ligne de commande.
- ``Application`` et ``Config`` sont importés depuis le module config, et sont utilisés pour la gestion de la configuration et l'application.

## Structure ``Flags``
```
/// Program that renders 3d objects
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Flags {
    /// Config file to use
    #[arg(short, long)]
    pub config: PathBuf,
}
```
- ``Flags`` définit la structure des arguments de ligne de commande.
- ``#[derive(Parser, Debug)]`` dérive les implémentations pour la gestion des arguments de ligne de commande et le débogage.
- Le champ config est défini avec les attributs ``short`` et ``long``, permettant de spécifier le fichier de configuration via ``-c`` ou ``--config``.

## Enum ``Error``
```
#[derive(Debug)]
pub enum Error {
    FailedToReadFile(Box<dyn std::error::Error>),
    FailedToParse(ron::de::SpannedError),
}
```
- ``Error`` est une énumération définissant les types d'erreurs pouvant survenir :
    - ``FailedToReadFile`` est utilisé lorsqu'il y a un problème pour lire le fichier de configuration.
    - ``FailedToParse`` est utilisé lorsqu'il y a une erreur de parsing avec ron.

## Méthode ``get_application``
```
impl Flags {
    pub fn get_application(self) -> Result<Application, Error> {
        let raw_config = std::fs::read_to_string(self.config)
            .map_err(|v| Error::FailedToReadFile(Box::new(v)))?;

        Ok(ron::from_str::<Config>(&raw_config)
            .map_err(Error::FailedToParse)?
            .process())
    }
}
```
- ``get_application`` est une méthode qui lit le fichier de configuration spécifié, le parse et retourne une instance d'Application.
- ``std::fs::read_to_string`` lit le contenu du fichier de configuration. En cas d'erreur, elle est convertie en ``Error::FailedToReadFile``.
- ``ron::from_str`` parse le contenu du fichier de configuration en une instance de ``Config``. En cas d'erreur de parsing, elle est convertie en ``Error::FailedToParse``.
    Enfin, Config::process() est appelé pour créer l'instance d'Application à partir de la configuration.

## Conclusion
Ce code est un composant clé pour une application de rendu 3D en ligne de commande. Il permet de spécifier un fichier de configuration via les arguments de ligne de commande, puis lit et analyse ce fichier pour configurer l'application. En cas d'erreur, il renvoie des erreurs spécifiques pour faciliter le débogage.
# flag.rs
# hit.rs
# main.rs
# material.rs
# plane_surf.rs
# ray.rs
# sphere.rs
# vec3.rs


