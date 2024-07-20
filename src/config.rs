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

#[typetag::serde]
pub trait UnprocessedData: Debug {
    fn process(&self) -> Box<dyn Hittable>;
}

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

#[derive(Debug)]
pub struct Application {
    pub world: HittableList,
    pub camera: Camera,
    pub light: i32,
    pub samples: i32,
    pub height: u32,
    pub width: u32,
}
