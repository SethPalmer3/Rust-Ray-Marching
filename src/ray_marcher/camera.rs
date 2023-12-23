use super::{Point3D, Vector3D};

pub struct Camera{
    pub position: Point3D,
    pub view_direction: Vector3D,
    pub near_plane_dist: f64,
    pub camera_angle: f64, // The angle from center line the camera can view (horizontal)
    pub resolution: (u32, u32),
}

impl Camera {
    pub fn new(position: Point3D, view_direction: Vector3D, near_plane_dist: f64, camera_angle: f64, resolution: (u32, u32)) -> Self{
        Camera { position, view_direction, near_plane_dist, camera_angle, resolution }
    }

    pub fn get_near_plane_point(&self, r: u32, c: u32) -> (Point3D, Vector3D){
        let horz_theta: f64 = self.camera_angle * (((r as f64 * 2.0) / self.resolution.0 as f64) - 1.0);
        let horz_delta = Vector3D::new(
            horz_theta.cos(),
            0.0,
            horz_theta.sin()
        );
        let aspect_ratio = (self.resolution.1 - 1) as f64 / (self.resolution.0 - 1) as f64;
        let camera_angle_adj = self.camera_angle * aspect_ratio;
        let vert_theta: f64 = camera_angle_adj * (1.0 - ((2.0 * c as f64) / self.resolution.1 as f64));
        let vert_delta = Vector3D::new(
            vert_theta.cos(),
            vert_theta.sin(),
            0.0
        );
        let d = (self.view_direction.clone() + horz_delta + vert_delta - super::Const_3D::X_DIR).get_norm();
        let vector_angle = Vector3D::get_dot(&self.view_direction, &d) / (self.view_direction.length() * d.length());
        let projection_dist = self.near_plane_dist / vector_angle;
        let p = (d.clone() * projection_dist).to_point();

        (p, d)
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use super::super::{Point3D, Vector3D};

    #[test]
    fn test_near_plane_point_direction(){
        let camera = Camera::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0), 1.0, 45_f64.to_radians(), (10,1));
        assert_eq!(camera.get_near_plane_point(0, 0).1, Vector3D::new(1.0, 0.0, -1.0));
    }
}
