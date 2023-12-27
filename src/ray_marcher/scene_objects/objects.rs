// use std::cmp;
use super::{Point3D, SceneObject, SurfaceMaterial, DEFAULT_SURFACEMAT};

#[derive(Clone)]
pub struct Sphere {
    radius: f64,
    position: Point3D,
    material: SurfaceMaterial,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(pos: Point3D, radius: f64, sm: Option<SurfaceMaterial>) -> Self {
        Sphere {
            radius,
            position: pos,
            material: sm.unwrap_or(DEFAULT_SURFACEMAT),
        }
    }
}

impl SceneObject for Sphere {
    fn signed_distance(&self, p: &Point3D) -> f64 {
        p.distance_to(&self.position) - self.radius
    }

    fn get_position(&self) -> &Point3D {
        &self.position
    }

    fn get_surface_material(&self) -> SurfaceMaterial {
        self.material.clone()
    }
}

#[cfg(test)]
mod test {
    use super::super::Vector3D;
    use super::*;

    #[test]
    fn test_sphere_sdf_x() {
        let s = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, None);
        let p = Point3D::new(2.0, 0.0, 0.0);
        assert_eq!(s.signed_distance(&p), 1.0);
    }
    #[test]
    fn test_sphere_sdf_y() {
        let s = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, None);
        let p = Point3D::new(0.0, 2.0, 0.0);
        assert_eq!(s.signed_distance(&p), 1.0);
    }
    #[test]
    fn test_sphere_sdf_z() {
        let s = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, None);
        let p = Point3D::new(0.0, 0.0, 2.0);
        assert_eq!(s.signed_distance(&p), 1.0);
    }
    #[test]
    fn test_sphere_sdf_inner() {
        let s = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, None);
        let p = Point3D::new(0.0, 0.0, 0.0);
        assert_eq!(s.signed_distance(&p), -1.0);
    }
    #[test]
    fn test_surface_normal_x() {
        let s = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, None);
        let p = Point3D::new(1.0, 0.0, 0.0);
        assert!(
            (Vector3D::new(1.0, 0.0, 0.0) - s.get_surface_normal(&p, 0.0001))
                .length()
                .abs()
                < 0.001
        );
    }
    #[test]
    fn test_surface_normal_y() {
        let s = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, None);
        let p = Point3D::new(0.0, 1.0, 0.0);
        assert!(
            (Vector3D::new(0.0, 1.0, 0.0) - s.get_surface_normal(&p, 0.0001))
                .length()
                .abs()
                < 0.001
        );
    }
    #[test]
    fn test_surface_normal_z() {
        let s = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, None);
        let p = Point3D::new(0.0, 0.0, 1.0);
        assert!(
            (Vector3D::new(0.0, 0.0, 1.0) - s.get_surface_normal(&p, 0.0001))
                .length()
                .abs()
                < 0.001
        );
    }
    #[test]
    fn test_surface_normal_xy() {
        let s = Sphere::new(Point3D::new(0.0, 0.0, 0.0), 1.0, None);
        let p = Point3D::new(1.0, 1.0, 0.0);
        assert!(
            (Vector3D::new(1.0, 1.0, 0.0).get_norm() - s.get_surface_normal(&p, 0.0001))
                .length()
                .abs()
                < 0.001
        );
    }
}
