use super::color::Color;
use super::geometry::Geometry;
use super::geometry::HitInfo;
use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cube {
    center: Vec3,
    color: Color,
    half_size: Vec3,
    reflection_factor: Option<f32>,
    transparency_factor: Option<f32>,
}

impl Cube {
    pub fn new(
        center: Vec3,
        half_size: Vec3,
        color: Color,
        reflection_factor: f32,
        transparency_factor: f32,
    ) -> Cube {
        Cube {
            center,
            half_size,
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

    pub fn get_half_size(&self) -> Vec3 {
        self.half_size
    }
}

impl Geometry for Cube {
    fn compute_hit(
        &self,
        ray: &Ray,
        hitinfo: Option<&mut HitInfo>,
        exit_dist: Option<&mut f32>,
    ) -> Option<f32> {
        let inv_dir = ray.get_direction().reciprocal(); // Composantes réciproques du vecteur direction du rayon
        let min = self.center - self.half_size;
        let max = self.center + self.half_size;

        let mut tmin = (min.x - ray.get_origin().x) * inv_dir.x;
        let mut tmax = (max.x - ray.get_origin().x) * inv_dir.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (min.y - ray.get_origin().y) * inv_dir.y;
        let mut tymax = (max.y - ray.get_origin().y) * inv_dir.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (min.z - ray.get_origin().z) * inv_dir.z;
        let mut tzmax = (max.z - ray.get_origin().z) * inv_dir.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        if tmin < 0.0 && tmax < 0.0 {
            return None;
        }

        let hit_distance = if tmin < 0.0 { tmax } else { tmin };

        if let Some(exit_dist) = exit_dist {
            *exit_dist = tmax;
        }

        if let Some(hit_info) = hitinfo {
            hit_info.position = ray.point_at(hit_distance);
            let normal = match hit_info.position {
                _ if (hit_info.position.x - min.x).abs() < 0.0001 => Vec3::new(-1.0, 0.0, 0.0),
                _ if (hit_info.position.x - max.x).abs() < 0.0001 => Vec3::new(1.0, 0.0, 0.0),
                _ if (hit_info.position.y - min.y).abs() < 0.0001 => Vec3::new(0.0, -1.0, 0.0),
                _ if (hit_info.position.y - max.y).abs() < 0.0001 => Vec3::new(0.0, 1.0, 0.0),
                _ if (hit_info.position.z - min.z).abs() < 0.0001 => Vec3::new(0.0, 0.0, -1.0),
                _ if (hit_info.position.z - max.z).abs() < 0.0001 => Vec3::new(0.0, 0.0, 1.0),
                _ => Vec3::new(0.0, 0.0, 0.0),
            };
            hit_info.normal = normal;
        }

        Some(hit_distance)
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
