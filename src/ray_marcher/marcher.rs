use super::color_data_types::Color;
use super::scene::{Scene, ClosestObject};
use super::scene_objects::{objects, SceneObject};
use super::ray;
use super::camera;

#[allow(dead_code)]
const RAY_STEPS: i32 = 3;
#[allow(dead_code)]
const MIN_HIT_DIST: f64 = 1e-12;
#[allow(dead_code)]
const EPSILON: f64 = 1e-7;


#[allow(dead_code)]
pub struct MarcherHandler{
    num_bounces: u32,
    rays: Vec<ray::Ray>,
    scene: Scene<objects::Sphere>,
    camera: camera::Camera,
    pub debug: bool
}

#[allow(dead_code)]
impl MarcherHandler {

    pub fn new(num_bounces: u32, camera: camera::Camera) -> Self {
        let mut ret = MarcherHandler { num_bounces, rays: Vec::<ray::Ray>::new(), scene: Scene::new(), camera, debug: false };
        ret.generate_rays();
        return ret;
    }

    pub fn add_scene_object(&mut self, o: objects::Sphere){
        self.scene.add_scene_object(o);
    }

    fn add_ray(&mut self, r: ray::Ray){
        self.rays.push(r);
    }

    fn generate_rays(&mut self){
        for c in 0..self.camera.resolution.1{
            for r in 0..self.camera.resolution.0{
                let (p, d) = self.camera.get_near_plane_point(r, c);
                self.add_ray(ray::Ray::new(p, d));
            }
        }
    }

    pub fn march(&mut self){
        loop{
            for ray in self.rays.iter_mut(){
                let closest_obj = self.scene.get_closest_object(ray.get_position());
                if let Some(ClosestObject { distance, obj }) = closest_obj{
                    ray.step(distance);
                    if distance < MIN_HIT_DIST{
                        if self.debug {
                            let n = obj.get_surface_normal(&ray.get_position(), EPSILON).get_norm().to_point();
                            ray.color = Color::new(n.x, n.y, n.z);
                            ray.stop();
                        }else{
                            ray.color = Color::blend_colors(&obj.get_surface_material().color, &ray.color, 0.5);
                        }
                        ray.reflect(&obj.get_surface_normal(ray.get_position(), EPSILON));
                    }
                }
            }
            self.num_bounces -= 1;
            if self.num_bounces <= 0 {
                break;
            }
        }
    }

    pub fn get_color(&self, x: u32, y: u32) -> Color {
        let (rows, _cols) = self.camera.get_resolution();
        let index = (x * rows) + y;
        self.rays.get(index as usize).unwrap().get_color()
    }
}

#[cfg(test)]
mod test{
    use crate::ray_marcher::scene_objects::{objects::*, SurfaceMaterial};

    use super::*;
    use super::super::*;

    #[test]
    // #[ignore = "Could be computationally expensive"]
    fn test_march_one_pixel(){
        let camera = camera::Camera::new(Const_3D::ORIGIN, Const_3D::X_DIR, 0.1, 0.0, (1,1));
        let mut marcher = MarcherHandler::new(100, camera);
        let sphere = Sphere::new(Point3D::new(10.0, 0.0, 0.0), 1.0, Some(SurfaceMaterial{ color: Color::new(1.0, 0.0, 0.0)}));
        marcher.add_scene_object(sphere);
        marcher.march();
        let(r, _g, _b) = marcher.get_color(0, 0).get_u8_components();
        assert_eq!(r, 255 as u8);
    }
    #[test]
    // #[ignore = "Could be computationally expensive"]
    fn test_march_two_pixels(){
        let camera = camera::Camera::new(Const_3D::ORIGIN, Const_3D::X_DIR, 0.1, 1.0_f64.to_radians(), (2,1));
        let mut marcher = MarcherHandler::new(100, camera);
        let sphere = Sphere::new(Point3D::new(10.0, 0.0, 0.0), 1.0, Some(SurfaceMaterial{ color: Color::new(1.0, 0.0, 0.0)}));
        marcher.add_scene_object(sphere);
        marcher.march();
        let(r, _g, _b) = marcher.get_color(0, 0).get_u8_components();
        assert_eq!(r, 255 as u8);
    }
}
