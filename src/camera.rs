use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

pub struct CameraOrientation {
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
}

pub struct CameraSettings {
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub aperture: f64,
    pub focus_dist: f64,
}

impl Camera {
    pub fn new(s: CameraSettings, o: CameraOrientation) -> Camera {
        let theta = (std::f64::consts::PI / 180.0) * s.vfov;
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = s.aspect_ratio * viewport_height;

        let w = (&o.lookfrom - &o.lookat).unit_vector();
        let u = vec3::cross(&o.vup, &w).unit_vector();
        let v = vec3::cross(&w, &u);

        let origin = o.lookfrom;
        let horizontal = s.focus_dist * viewport_width * &u;
        let vertical = s.focus_dist * viewport_height * &v;
        let lower_left_corner =
            &origin - &horizontal / 2. - &vertical / 2. - s.focus_dist * w;

        let lens_radius = s.aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * vec3::random_in_unit_disk();
        let offset = &self.u * rd.x() + &self.v * rd.y();

        Ray::new(
            &self.origin + &offset,
            &self.lower_left_corner + s * &self.horizontal + t * &self.vertical - &self.origin - &offset,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ray::Ray;
    use crate::vec3::Vec3;

    fn get_camera(aspect_ratio: f64) -> Camera {
        let orientation = CameraOrientation {
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat:   Point3::new(0.0, 0.0, -1.0),
            vup:      Vec3::new(0.0, 1.0, 0.0),
        };
        let settings = CameraSettings {
            vfov: 90.0,
            aspect_ratio,
            aperture: 0.0,
            focus_dist: (&orientation.lookfrom - &orientation.lookat).length(),
        };
        Camera::new(settings, orientation)
    }

    #[test]
    fn test_camera_new() {
        let aspect_ratio = 2. / 1.;
        let camera = get_camera(aspect_ratio);

        assert_eq!(camera.origin, Point3::new(0., 0., 0.));
        assert_eq!(camera.horizontal, Vec3::new(4.0, 0.0, 0.0));
        assert_eq!(camera.vertical, Vec3::new(0.0, 2.0, 0.0));
        assert_eq!(camera.lower_left_corner, Vec3::new(-2., -1., -1.));
    }

    #[test]
    fn test_camera_get_ray() {
        let camera = get_camera(2.0 / 1.0);
        assert_eq!(
            camera.get_ray(2., 3.),
            Ray::new(Vec3::new(0., 0., 0.), Vec3::new(6.0, 5.0, -1.0))
        );
        assert_eq!(
            camera.get_ray(7., 14.),
            Ray::new(Vec3::new(0., 0., 0.), Vec3::new(26.0, 27.0, -1.0))
        );
        assert_eq!(
            camera.get_ray(13., 21.),
            Ray::new(Vec3::new(0., 0., 0.), Vec3::new(50.0, 41.0, -1.0))
        );
    }
}
