use crate::vec3::Vec3;
use crate::point::Point;
use crate::hittable::HittableList;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::util::INFINITY;

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub fn ray_color(r: Ray, world: &HittableList, depth: u64) -> Color {
    let mut hit_record : HitRecord = HitRecord::new();

    if depth <= 0 {
        return Color::new(0.0,0.0,0.0);
    }   

    if world.hit(&r, 0.001, INFINITY, &mut hit_record) {
        let target  = hit_record.p + hit_record.normal + Vec3::new_random_in_hemisphere(&hit_record.normal);
        return 0.5 * ray_color(Ray::new(hit_record.p, target - hit_record.p), world, depth-1);
    }

    let unit_direction = r.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0-t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}