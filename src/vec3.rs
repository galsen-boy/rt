use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub},
};

use rand::{distributions::Standard, prelude::*};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3(rng.gen(), rng.gen(), rng.gen())
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn r(self) -> f64 {
        self.0
    }
    pub fn g(self) -> f64 {
        self.1
    }
    pub fn b(self) -> f64 {
        self.2
    }

    pub fn x(self) -> f64 {
        self.0
    }
    pub fn y(self) -> f64 {
        self.1
    }
    pub fn z(self) -> f64 {
        self.2
    }

    pub fn length(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn squared_length(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
    }
    
    pub fn dot_j(&self, v2: &Vec3) -> f64 {
        self.0 * v2.0 + self.1 * v2.1 + self.2 * v2.2
    }

    pub fn dot_y(&self, other: &Vec3) -> f64 {
        self.1 * other.1
    }

    pub fn dot_xz(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.2 * other.2
    }

    pub fn dot_xy(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn dot_yz(&self, other: &Vec3) -> f64 {
        self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3(
            v1.1 * v2.2 - v1.2 * v2.1,
            v1.2 * v2.0 - v1.0 * v2.2,
            v1.0 * v2.1 - v1.1 * v2.0,
        )
    }

    pub fn random_init(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3(
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
        )
    }

    pub fn unit(self) -> Vec3 {
        self / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::default(), |acc, x| acc + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0),
            Vec3::new(5.0, 7.0, 9.0)
        );
    }

    #[test]
    fn test_vec3_mul() {
        assert_eq!(Vec3::new(5.0, 4.0, 1.0) * 2.0, Vec3::new(10.0, 8.0, 2.0));
    }

    #[test]
    fn test_vec3_div() {
        assert_eq!(Vec3::new(10.0, 8.0, 2.0) / 2.0, Vec3::new(5.0, 4.0, 1.0));
    }

    #[test]
    fn test_vec3_negate() {
        assert_eq!(-Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0));
    }
}
