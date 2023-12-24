use super::{Vector3D, Point3D, color_data_types::Color};

#[allow(dead_code)]
const MIN_HIT_DIST: f64 = 0.001;

#[allow(dead_code)]
pub struct Ray {
    position: Point3D,
    direction: Vector3D,
    num_hits: i32,
    must_stop: bool,
    pub color: Color,
}


impl Ray {
    pub fn new(pos: Point3D, dir: Vector3D) -> Self {
        Ray { position: pos, direction: dir, num_hits: 0, color: Color::new(0.0, 0.0, 0.0), must_stop: false }
    }
    pub fn step(&mut self, step_size: f64) {
        if !self.must_stop{
            self.position += self.direction.clone().to_point() * step_size;
        }
    }
    pub fn get_position(&self) -> &Point3D {
        &self.position
    }
    pub fn get_direction(&self) -> &Vector3D{
        &self.direction
    }
    pub fn get_num_hits(&self) -> i32 {
        self.num_hits
    }
    pub fn reflect(&mut self, surf_normal: &Vector3D) {
        let normal = surf_normal.get_norm();
        self.direction -= normal * 2.0 * Vector3D::get_dot(&self.direction, &normal);
        self.num_hits += 1;
    }
    pub fn stop(&mut self){
        self.must_stop = true;
    }
    pub fn has_stopped(&self) -> bool {
        self.must_stop
    }
    pub fn get_color(&self) -> Color{
        if self.num_hits > 0 {
            return self.color.clone();
        }
        Color::new(1.0, 1.0, 1.0)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use super::super::Const_3D;

    #[test]
    fn test_new(){
        let n_ray = Ray::new(Const_3D::ORIGIN, Const_3D::X_DIR);
        assert_eq!(*n_ray.get_position(), Const_3D::ORIGIN);
    }

    #[test]
    fn test_step() {
        let step_size = 10_f64;
        let mut n_ray = Ray::new(Const_3D::ORIGIN, Const_3D::X_DIR);
        n_ray.step(step_size);
        assert_eq!(*n_ray.get_position(), (Const_3D::X_DIR*step_size).to_point());
    }

    #[test]
    fn test_reflection(){
        let mut incoming = Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 1.0, 0.0).get_norm());
        let surf_norm = Vector3D::new(0.0, -1.0, 0.0);
        incoming.reflect(&surf_norm);
        assert_eq!(*incoming.get_direction(), Vector3D::new(1.0, -1.0, 0.0).get_norm());

    }

}
