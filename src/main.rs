pub mod ray_marcher;

use ray_marcher::{threed_data_types::{Point, Direction as Vector3D}, scene_objects::{objects::Sphere, SurfaceMaterial}, color_data_types::Color};
use image::{self, RgbImage};

fn main() {
    let camera = ray_marcher::camera::Camera::new(Point::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0).get_norm(), 1.0, 60_f64.to_radians(), (600, 600));
    let mut march_handler = ray_marcher::marcher::MarcherHandler::new(100, camera);
    // march_handler.debug = true;
    march_handler.add_scene_object(Sphere::new(Point::new(30.0, 0.0, 0.0), 10.0, Some(SurfaceMaterial{ color: Color::new(1.0, 0.5, 0.0)})));
    // march_handler.add_scene_object(Sphere::new(Point::new(20.0, 10.0, 0.0), 10.0, Some(SurfaceMaterial{ color: Color::new(0.0, 1.0, 0.0)})));
    march_handler.march();

    let mut image_buf: RgbImage = image::ImageBuffer::new(600, 600);

    for (x,y,pixel) in image_buf.enumerate_pixels_mut(){
        let col = march_handler.get_color(x, y);
        let (r, g, b) = col.get_u8_components();
        *pixel = image::Rgb([r, g, b]);
    }
    image_buf.save("ray_marched.png").unwrap();

}
