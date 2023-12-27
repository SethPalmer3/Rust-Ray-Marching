use std::f64::consts::PI;

use rand::Rng;

use rayon;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use super::color_data_types::Color;
use super::scene::{Scene, ClosestObject};
use super::scene_objects::{objects, SceneObject};
use super::ray;
use super::camera;

#[allow(dead_code)]
const MAX_HITS: i32 = 10;
#[allow(dead_code)]
pub const MIN_HIT_DIST: f64 = 1e-12;
#[allow(dead_code)]
pub const MAX_DISTANCE: f64 = 1e7;
#[allow(dead_code)]
const EPSILON: f64 = 1e-7;
#[allow(dead_code)]
const MIN_ANGLE: f64 = (0.1_f64 * 180_f64) / PI;


#[allow(dead_code)]
pub struct MarcherHandler{
    num_steps: u32,
    num_iterations: u32,
    max_distance: f64,
    rays: Vec<ray::Ray>,
    scene: Scene<objects::Sphere>,
    camera: camera::Camera,
    pub debug: bool
}

#[allow(dead_code)]
impl MarcherHandler {

    pub fn new(num_bounces: u32, max_distance: f64, num_iterations: u32, camera: camera::Camera) -> Self {
        let mut ret = MarcherHandler { num_steps: num_bounces, rays: Vec::<ray::Ray>::new(), scene: Scene::new(), camera, debug: false, max_distance, num_iterations };
        ret.generate_rays();
        return ret;
    }

    pub fn get_camera(&self) -> &camera::Camera{
        &self.camera
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
        let num_bounce_const = self.num_steps;
        loop{
            self.rays.par_iter_mut().for_each(|ray| {
                if ray.has_stopped() {
                    // println!("stopped - {}", i);
                    return;
                }
                let closest_obj = self.scene.get_closest_object(ray.get_position());
                if let Some(ClosestObject { distance, obj }) = closest_obj{
                    // println!("{} - {}", distance, i);
                    ray.step(distance);
                    if distance >= MAX_DISTANCE || ray.get_num_hits() > MAX_HITS{
                        ray.stop();
                        return;
                    }
                    if distance < MIN_HIT_DIST{
                        if self.debug {
                            let n = obj.get_surface_normal(&ray.get_position(), EPSILON).get_norm().to_point();
                            ray.color = Color::new(n.x, n.y, n.z);
                            ray.stop();
                        }else{
                            ray.color = Color::blend_colors(&obj.get_surface_material().color, &ray.color, 0.5);
                        }
                        ray.scatter(&obj.get_surface_normal(ray.get_position(), EPSILON), &obj.get_surface_material(), 120_f64.to_radians(), 0.001);
                    }
                }
            });
            self.num_steps -= 1;
            if self.num_steps <= 0 {
                self.num_iterations -= 1;
                self.num_steps = num_bounce_const;
                self.reset_rays();
                if self.num_iterations <= 0 {
                    break;
                }
            }
        }
    }

    pub fn get_color(&self, x: u32, y: u32) -> Color {
        let (rows, _cols) = self.camera.get_resolution();
        let index = (x * rows) + y;
        self.rays.get(index as usize).unwrap().get_color()
    }

    fn index_to_res_coords(cam_row: u32, cam_col: u32, i: usize) -> (u32, u32){
        let c = i as u32 / cam_col;
        let r = i as u32 % cam_row;
        (r, c)
    }

    fn reset_rays(&mut self){
        let (cam_row, cam_col) = self.camera.get_resolution();
        for (i, ray) in self.rays.iter_mut().enumerate(){
            let (r, c) = Self::index_to_res_coords(cam_row, cam_col, i);
            let (p, mut d) = self.camera.get_near_plane_point(r, c);

            let mut rng = rand::thread_rng();
            let rand_y = (rng.gen::<f64>() * 2.0) - 1.0;
            let rand_z = (rng.gen::<f64>() * 2.0) - 1.0;
            d.rotate_vector(rand_z * MIN_ANGLE, rand_y * MIN_ANGLE);
            ray.set_position(p);
            ray.set_direction(d);
        }
    }
}

#[cfg(test)]
mod test{
    use crate::ray_marcher::scene_objects::{objects::*, SurfaceMaterial};

    use super::*;
    use super::super::*;

    #[test]
    fn test_index_to_coords_r(){
        let (r, _c) = MarcherHandler::index_to_res_coords(100, 100, 2);
        assert_eq!(r, 2);
    }

    #[test]
    fn test_index_to_coords_c(){
        let (_r, c) = MarcherHandler::index_to_res_coords(100, 100, 2);
        assert_eq!(c, 0);
    }

    #[test]
    // #[ignore = "Could be computationally expensive"]
    fn test_march_one_pixel(){
        let camera = camera::Camera::new(Const_3D::ORIGIN, Const_3D::X_DIR, 0.1, 0.0, (1,1));
        let mut marcher = MarcherHandler::new(100, MAX_DISTANCE, 1, camera);
        let sphere = Sphere::new(Point3D::new(10.0, 0.0, 0.0), 1.0, Some(SurfaceMaterial{ color: Color::new(1.0, 0.0, 0.0), reflectivity: 1.0 }));
        marcher.add_scene_object(sphere);
        marcher.march();
        let(r, _g, _b) = marcher.get_color(0, 0).get_u8_components();
        assert_eq!(r, 255 as u8);
    }
    #[test]
    // #[ignore = "Could be computationally expensive"]
    fn test_march_two_pixels(){
        let camera = camera::Camera::new(Const_3D::ORIGIN, Const_3D::X_DIR, 0.1, 1.0_f64.to_radians(), (2,1));
        let mut marcher = MarcherHandler::new(100, MAX_DISTANCE, 1, camera);
        let sphere = Sphere::new(Point3D::new(10.0, 0.0, 0.0), 1.0, Some(SurfaceMaterial{ color: Color::new(1.0, 0.0, 0.0), reflectivity: 1.0 }));
        marcher.add_scene_object(sphere);
        marcher.march();
        let(r, _g, _b) = marcher.get_color(0, 0).get_u8_components();
        assert_eq!(r, 255 as u8);
    }
}
