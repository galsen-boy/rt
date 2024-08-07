use super::color::Color;
use super::geometry::Geometry;
use super::geometry::HitInfo;
use super::ray::Ray;
use super::vec3::Vec3;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cylinder {
    center: Vec3,
    color: Color,
    radius: f32,
    height: f32,
    reflection_factor: Option<f32>,
    transparency_factor: Option<f32>,
}

impl Cylinder {
    pub fn new(
        center: Vec3,
        radius: f32,
        height: f32,
        color: Color,
        reflection_factor: f32,
        transparency_factor: f32,
    ) -> Cylinder {
        Cylinder {
            center,
            radius,
            height,
            color,
            reflection_factor: if reflection_factor > 0.001 {
                Some(reflection_factor)
            } else {
                None
            },
            transparency_factor: if transparency_factor > 0.001 {
                Some(transparency_factor)
            } else {
                None
            },
        }
    }

    pub fn get_center(&self) -> Vec3 {
        self.center
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }
}

impl Geometry for Cylinder {
    fn compute_hit(
        &self,
        ray: &Ray,
        hitinfo: Option<&mut HitInfo>,
        exit_dist: Option<&mut f32>,
    ) -> Option<f32> {
        let ray_to_cylinder = ray.get_origin() - self.center;
        let a = Vec3::dot_product(ray.get_direction(), ray.get_direction()) - ray.get_direction().z.powi(2);
        let b = Vec3::dot_product(ray.get_direction(), &ray_to_cylinder) - ray.get_direction().z * ray_to_cylinder.z;
        let c = Vec3::dot_product(&ray_to_cylinder, &ray_to_cylinder) - ray_to_cylinder.z.powi(2) - self.radius.powi(2);

        let delta = b * b - a * c;

        let compute_result = |param: f32, hit_info: &mut HitInfo| {
            hit_info.position = ray.point_at(param);
            let normal = Vec3::new(
                hit_info.position.x - self.center.x,
                hit_info.position.y - self.center.y,
                0.0
            ).normalize();
            hit_info.normal = normal;
        };

        if delta >= 0.0 {
            let sqrt_delta = delta.sqrt();
            let mut enter_distance = (-b - sqrt_delta) / a;
            let mut exit_distance = (-b + sqrt_delta) / a;

            if enter_distance > exit_distance {
                std::mem::swap(&mut enter_distance, &mut exit_distance);
            }

            let hit_position = ray.point_at(enter_distance);
            if hit_position.z < self.center.z || hit_position.z > self.center.z + self.height {
                return None;
            }

            if enter_distance >= 0.0 {
                if let Some(exit_dist) = exit_dist {
                    *exit_dist = exit_distance;
                }

                if let Some(hit_info) = hitinfo {
                    compute_result(enter_distance, hit_info);
                }

                return Some(enter_distance);
            }
        }

        None
    }

    fn get_color(&self, position: &Vec3) -> Color {
        self.get_color()
    }

    fn get_reflection_factor(&self) -> Option<f32> {
        self.reflection_factor
    }

    fn get_transparency_factor(&self) -> Option<f32> {
        self.transparency_factor
    }
}
