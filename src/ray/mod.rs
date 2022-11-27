use crate::vec3;

use vec3::Vec3 as Point3;
pub struct Ray {
    orig: Point3,
    dir: vec3::Vec3
}

impl Ray {
    pub fn new(orig: Point3, dir: vec3::Vec3) -> Self { Self { orig, dir } }

    pub fn origin(&self) -> Point3 { self.orig }
    pub fn direction(&self) -> vec3::Vec3 { self.dir }

    pub fn at(&self, t: f64) -> Point3 { self.orig + self.dir * t }

}