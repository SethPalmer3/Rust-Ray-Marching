pub mod ray_marcher;

use image::{self, RgbImage};
use ray_marcher::{
    color_data_types::{f64_to_u8, Color},
    marcher,
    scene_objects::{objects::Sphere, SurfaceMaterial},
    screen::Displayable,
    threed_data_types::{Direction as Vector3D, Point},
};

fn main() {
    let height_res = 600;
    let width_res = 600;
    let camera = ray_marcher::camera::Camera::new(
        Point::new(0.0, 0.0, 0.0),
        Vector3D::new(1.0, 0.0, 0.0).get_norm(),
        1.0,
        60_f64.to_radians(),
        (width_res, height_res),
    );
    let mut march_handler =
        ray_marcher::marcher::MarcherHandler::new(1000, marcher::MAX_DISTANCE, 50, camera);
    // march_handler.debug = true;
    march_handler.add_scene_object(Sphere::new(
        Point::new(30.0, -10.0, 0.0),
        10.0,
        Some(SurfaceMaterial {
            color: Color::new(1.0, 0.0, 0.0),
            reflectivity: 1.0,
        }),
    ));
    march_handler.add_scene_object(Sphere::new(
        Point::new(30.0, 10.0, 0.0),
        10.0,
        Some(SurfaceMaterial {
            color: Color::new(0.0, 0.0, 1.0),
            reflectivity: 0.0,
        }),
    ));
    let screen = march_handler.march();

    let mut image_buf: RgbImage = image::ImageBuffer::new(
        march_handler.get_camera().resolution.0,
        march_handler.get_camera().resolution.1,
    );

    for (x, y, pixel) in image_buf.enumerate_pixels_mut() {
        let (r, g, b) = screen.get_color_components((x, y));
        *pixel = image::Rgb([f64_to_u8(r), f64_to_u8(g), f64_to_u8(b)]);
    }
    image_buf.save("ray_marched.png").unwrap();
}
