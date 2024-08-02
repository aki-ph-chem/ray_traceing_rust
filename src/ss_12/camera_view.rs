use ray_tracing_rust::camera::Camera;
use ray_tracing_rust::color::Color;
use ray_tracing_rust::hittable_list::HittableList;
use ray_tracing_rust::material::{Lambertian, Material};
use ray_tracing_rust::sphere_material::SphereMat;
use ray_tracing_rust::utl;
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

    let material_left: Option<Rc<RefCell<dyn Material>>> = Some(Rc::new(RefCell::new(
        Lambertian::new(&Color::from_slice([0.0, 0.0, 1.0])),
    )));
    let material_right: Option<Rc<RefCell<dyn Material>>> = Some(Rc::new(RefCell::new(
        Lambertian::new(&Color::from_slice([1.0, 0.0, 0.0])),
    )));

    let r = (utl::constans::PI / 4.0).cos();
    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([-r, 0.0, -1.0]),
        r,
        material_left,
    ))));
    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([r, 0.0, -1.0]),
        r,
        material_right,
    ))));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 90.0;

    let gamma = 0.50;
    camera.render_material(gamma, &world, &file_name)?;

    Ok(())
}
