use super::{Vector3D, Point3D, color_data_types::Color};

#[allow(dead_code)]
const MIN_HIT_DIST: f64 = 0.001;

#[allow(dead_code)]
pub struct Ray {
    position: Point3D,
    direction: Vector3D,
    remaining_step: i32,
    pub color: Color,
}


impl Ray {
    pub fn new(pos: Point3D, dir: Vector3D, steps: i32) -> Self {
        Ray { position: pos, direction: dir, remaining_step: steps, color: Color::new(1.0, 1.0, 1.0) }
    }
    pub fn step(&mut self, step_size: f64) {
        if self.remaining_step > 0 {
            self.remaining_step -= 1;
            self.position += self.direction.clone().to_point() * step_size;
        }
    }
    pub fn get_position(&self) -> &Point3D {
        &self.position
    }
    pub fn get_direction(&self) -> &Vector3D{
        &self.direction
    }
    pub fn get_remaining_steps(&self) -> i32 {
        self.remaining_step
    }
    pub fn reflect(&mut self, surf_normal: &Vector3D) {
        let normal = surf_normal.get_norm();
        self.direction -= normal * 2.0 * Vector3D::get_dot(&self.direction, &normal);
    }
    pub fn stop(&mut self){
        self.remaining_step = 0;
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use super::super::Const_3D;

    #[test]
    fn test_new(){
        let n_ray = Ray::new(Const_3D::ORIGIN, Const_3D::X_DIR, 1);
        assert_eq!(*n_ray.get_position(), Const_3D::ORIGIN);
    }

    #[test]
    fn test_step() {
        let step_size = 10_f64;
        let mut n_ray = Ray::new(Const_3D::ORIGIN, Const_3D::X_DIR, 1);
        n_ray.step(step_size);
        assert_eq!(*n_ray.get_position(), (Const_3D::X_DIR*step_size).to_point());
    }

    #[test]
    fn test_reflection(){
        let mut incoming = Ray::new(Point3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 1.0, 0.0).get_norm(), 1);
        let surf_norm = Vector3D::new(0.0, -1.0, 0.0);
        incoming.reflect(&surf_norm);
        assert_eq!(*incoming.get_direction(), Vector3D::new(1.0, -1.0, 0.0).get_norm());

    }

}
