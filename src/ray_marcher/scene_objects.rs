pub mod objects;
use super::Point3D;
use super::Vector3D;
use super::color_data_types::BLACK;
use super::color_data_types::Color;

pub trait SceneObject {
    fn signed_distance(&self, p: &Point3D) -> f64; // A minimum distance between the object and a point
    fn get_position(&self) -> &Point3D;
    fn get_surface_normal(&self, p: &Point3D, epsilon: f64) -> Vector3D{
        let center = self.signed_distance(p);

        let x_off = self.signed_distance(&(p.clone() + Point3D::new(epsilon, 0_f64, 0_f64)));
        let y_off = self.signed_distance(&(p.clone() + Point3D::new(0_f64, epsilon, 0_f64)));
        let z_off = self.signed_distance(&(p.clone() + Point3D::new(0_f64, 0_f64, epsilon)));
        (Point3D::new(x_off, y_off, z_off) - center).to_direction() / epsilon
    }
    fn get_surface_material(&self) -> SurfaceMaterial;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct SurfaceMaterial {
    pub color: Color,
    pub reflectivity: f64,
}
pub static DEFAULT_SURFACEMAT: SurfaceMaterial = SurfaceMaterial{ color: BLACK, reflectivity: 1.0 };
