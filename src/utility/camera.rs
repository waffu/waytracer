use super::{ray::Ray, vec3::*};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {

        // TODO: move these settings into the settings.toml

        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width: f32 = viewport_height * aspect_ratio;
        
        let focal_length = 1.0;

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let origin = Point3::new(0.0, 0.0, 0.0);

        Self {
            origin: Point3::new(0.0, 0.0, 0.0),
            lower_left_corner: origin
                - (horizontal / 2.0)
                - (vertical / 2.0)
                - Vec3::new(0.0, 0.0, focal_length),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
