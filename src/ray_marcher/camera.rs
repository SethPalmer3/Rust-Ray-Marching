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
        let pos_delta = self.view_direction * self.near_plane_dist; // init direction

        let mut horz_vector = pos_delta.clone();
        horz_vector.rotate_vector_around_y(-90_f64.to_radians());
        horz_vector.set_norm();
        let mut vert_vector = pos_delta.clone();
        vert_vector.rotate_vector_around_z(90_f64.to_radians());
        vert_vector.set_norm();

        let horz_vector_len_delta = self.camera_angle * 
            (((2.0 * r as f64) /
            self.resolution.0 as f64) - 
            1.0);
        let vert_cam_angle = self.camera_angle * (self.resolution.1 as f64 / self.resolution.0 as f64);
        let vert_vector_len_delta = vert_cam_angle * (1.0 - ((2.0 * c as f64) / self.resolution.1 as f64));

        horz_vector *= horz_vector_len_delta;
        vert_vector *= vert_vector_len_delta;
        let p = (pos_delta + horz_vector + vert_vector).to_point();
        let d = (p - self.position).to_direction().get_norm();

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
    fn test_near_plane_point_direction_single_pixel(){
        let camera = Camera::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0), 1.0, 0_f64.to_radians(), (1,1));
        assert_eq!(camera.get_near_plane_point(0, 0).1, Vector3D::new(1.0, 0.0, 0.0).get_norm());
    }

    #[test]
    fn test_near_plane_point_direction_double_pixel(){
        let camera = Camera::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0), 1.0, 45_f64.to_radians(), (2,1));
        assert_eq!(camera.get_near_plane_point(0, 0).1, Vector3D::new(1.0, 0.0, -1.0).get_norm());
    }
}
