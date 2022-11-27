/// 3D vector
use crate::randomizer::{rand_f64, rand_f64_range};
use std::{ops, fmt};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

// Operator -
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}

//Operator +=
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] = self.e[0] + rhs.e[0];
        self.e[1] = self.e[1] + rhs.e[1];
        self.e[2] = self.e[2] + rhs.e[2];
    }
}

// Operator *=
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

// Operator /=
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] *= 1.0/rhs;
        self.e[1] *= 1.0/rhs;
        self.e[2] *= 1.0/rhs;
    }
}

// Operator []
impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

// Operator +
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2] ] }
    }
}

// Operator -
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2] ] }
    }
}

// Operator *
impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self { e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs ] }
    }
}

// Operator /
impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self { e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs ] }
    }
}
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 { e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs ] }
    }
}

// Printing
impl fmt::Display for &Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Vec3 {    
    pub fn new(a: f64, b: f64, c: f64) -> Self { Self { e: [a,b,c] }}

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length_squared(&self) -> f64 { self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2] }
    pub fn length(&self) -> f64 { self.length_squared().sqrt() }

    // dot product
    pub fn dot(&self, rhs: &Self) -> f64 { self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2] }

    // cross product
    pub fn cross(&self, rhs: &Self) -> Self { Self {
            e: [self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0]]
        }
    }

    // unit vector
    pub fn unit_vec(&self) -> Self { self / self.length() }

    pub fn random() -> Self { Self { e: [rand_f64(), rand_f64(), rand_f64()] } }
    pub fn random_range(min: f64, max: f64) -> Self { Self { e: [rand_f64_range(min, max), rand_f64_range(min, max), rand_f64_range(min, max)] } }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 { 
                return p;
            }
        }
    }
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vec()
    }
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_y_z() {
        let v = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!((0.0, 1.0, 2.0), (v.e[0], v.e[1], v.e[2]));
    }

    #[test]
    fn length_test() {
        let v = Vec3::new(2.0, 2.0, 2.0);
        assert!(((12.0_f64).sqrt() - v.length()).abs() < f64::EPSILON);
    }

    #[test]
    fn dot_test_self() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert!((14.0 - v.dot(&v.clone())).abs() < f64::EPSILON);
    }
    
    #[test]
    fn dot_test_other() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        assert!((20.0 - v1.dot(&v2)).abs() < f64::EPSILON);
    }

}