use ray_tracing_rust::camera::Camera;
use ray_tracing_rust::hittable_list::HittableList;
use ray_tracing_rust::sphere::Sphere;
use ray_tracing_rust::vec3::Point3;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn Error>> {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() < 2 {
        panic!("Error: invalid args");
    }
    let file_name = argv[1].clone();

    let mut world = HittableList::new();
    world.add(Rc::new(RefCell::new(Sphere::new(
        &Point3::from_slice([0.0, 0.0, -1.0]),
        0.5,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        &Point3::from_slice([0.0, -100.5, -1.0]),
        100.0,
    ))));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    let gamma = 0.50;
    camera.render_gamma(gamma, &world, &file_name)?;

    Ok(())
}
