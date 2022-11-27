use crate::ray;
use crate::vec3;

use vec3::Vec3 as Point3;

pub struct HitRecord {
    pub p : Point3,
    pub normal: vec3::Vec3,
    pub t: f64,
    pub front_face: bool,
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self { Self { objects: Vec::new() } }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, obj: impl Hittable + 'static) {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut obj_hit;
        let mut closest = t_max;
        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest) {
                closest = hit.t;
                obj_hit = hit;
                hit_anything = Some(obj_hit);
            }
        }
        hit_anything
    }
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(p: Point3, normal: vec3::Vec3, t: f64, front_face: bool) -> Self { Self { p, normal, t, front_face } }

    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &vec3::Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

