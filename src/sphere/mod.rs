use crate::hittable;
use crate::hittable::Hittable;
use crate::vec3;

use vec3::Vec3 as Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self { Self { center, radius } }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }
        // Find the nearest root that lies in the acceptable range.
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if (root < t_min) || (root > t_max) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (root > t_max) {
                return None;
            }
        }
        let rec_t = root;
        let rec_p = r.at(root);
        let mut record = hittable::HitRecord::new(rec_p, (rec_p - self.center) / self.radius, rec_t, false);
        let outward_normal = (rec_p - self.center) / self.radius;
        record.set_face_normal(r, &outward_normal);
        Some(record)
    }
}
