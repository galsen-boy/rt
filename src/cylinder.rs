use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Cylinder {
    center: Vec3,
    radius: f64,
    height: f64,
    material: Material,
    
}

// #[derive(Debug)]
// pub struct XZCircle {
//     center: Vec3,
//     radius: f64,
//     height: f64,
//     material: Material
// }

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

// https://www.youtube.com/watch?v=UTz7ytMJ2yk
impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let t = (0.0001 - r.origin.y()) / r.direction.y();
        let oc = r.origin - self.center;

        let a = r.direction.dot_xz(&r.direction);
        let b = oc.dot_xz(&r.direction);
        let c = oc.dot_xz(&oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        let mut t_array: [f64; 4] = [0.0; 4];
        let mut valid_array: [bool; 4] = [false; 4];
        // point of intersection
        let mut poi_array: [Option<HitRecord>; 4] = [None; 4];



        t_array[2] = (self.center.y() - r.origin.y()) / r.direction.y();
        t_array[3] = (self.center.y() + self.height - r.origin.y()) / r.direction.y();



        let mut poi = oc + t_array[2] * r.direction;

        let mut normal = (r.at(t_array[2]) - self.center) / self.radius;

        if t_array[2] > t_min && t_array[2] < t_max && poi.dot_xz(&poi) < self.radius * self.radius{ 
            valid_array[2] = true;
            poi_array[2] = Some(HitRecord {
                t: t_array[2],
                point: r.at(t_array[2]),
                normal,
                u: 0.0,
                v: 0.0,
                material: self.material,
            });
        } else {
            t_array[2] = 100e6;
        }

        
        poi = oc + t_array[3] * r.direction;

        normal = (r.at(t_array[3]) - self.center) / self.radius;

        if t_array[3] > t_min && t_array[3] < t_max && poi.dot_xz(&poi) < self.radius * self.radius{ 
            valid_array[3] = true;
            poi_array[3] = Some(HitRecord {
                t: t_array[3],
                point: r.at(t_array[3]),
                normal,
                u: 0.0,
                v: 0.0,
                material: self.material,
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
                    material: self.material,
                });
                // }
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
                    material: self.material,
                });
            } else {
                t_array[1] = 100e6;
            }
        }

        if !valid_array[0] && !valid_array[1] && !valid_array[2] && !valid_array[3] {
            return None
        }

        let mut min_index = 0;
        let mut min_value = 10e6;
        for (i, value) in t_array.iter().enumerate() {
            if value < &min_value
            {
                min_value = *value;
                min_index = i;
            }
        }

        let valid_poi = poi_array[min_index];
        return valid_poi;

    }
}

