use crate::{
    hittable::Ray,
    vec3::{Point3, Scalar, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: Scalar) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: Scalar, v: Scalar) -> Ray {
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;

        Ray {
            origin: self.origin,
            direction,
        }
    }
}
