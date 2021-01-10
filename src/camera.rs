use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner =
            &origin - &horizontal / 2. - &vertical / 2. - Vec3::new(0., 0., focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &self.lower_left_corner + u * &self.horizontal + v * &self.vertical - &self.origin,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ray::Ray;
    use crate::vec3::Vec3;

    #[test]
    fn test_camera_new() {
        let aspect_ratio = 2. / 1.;
        let camera = Camera::new(aspect_ratio);

        assert_eq!(camera.origin, Point3::new(0., 0., 0.));
        assert_eq!(camera.horizontal, Vec3::new(4.0, 0.0, 0.0));
        assert_eq!(camera.vertical, Vec3::new(0.0, 2.0, 0.0));
        assert_eq!(camera.lower_left_corner, Vec3::new(-2., -1., -1.));
    }

    #[test]
    fn test_camera_get_ray() {
        let camera = Camera::new(2.0 / 1.0);
        assert_eq!(
            camera.get_ray(2., 3.),
            Ray::new(&Vec3::new(0., 0., 0.), Vec3::new(6.0, 5.0, -1.0))
        );
        assert_eq!(
            camera.get_ray(7., 14.),
            Ray::new(&Vec3::new(0., 0., 0.), Vec3::new(26.0, 27.0, -1.0))
        );
        assert_eq!(
            camera.get_ray(13., 21.),
            Ray::new(&Vec3::new(0., 0., 0.), Vec3::new(50.0, 41.0, -1.0))
        );
    }
}
