use ray_tracing_rust::camera::Camera;
use ray_tracing_rust::color::Color;
use ray_tracing_rust::hittable_list::HittableList;
use ray_tracing_rust::material::{DielectricV3, Lambertian, Material, MetalFuzz};
use ray_tracing_rust::sphere_material::SphereMat;
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

    let material_ground: Option<Rc<RefCell<dyn Material>>> = Some(Rc::new(RefCell::new(
        Lambertian::new(&Color::from_slice([0.8, 0.8, 0.0])),
    )));
    let material_center: Option<Rc<RefCell<dyn Material>>> = Some(Rc::new(RefCell::new(
        Lambertian::new(&Color::from_slice([0.1, 0.2, 0.5])),
    )));
    let material_left: Option<Rc<RefCell<dyn Material>>> =
        Some(Rc::new(RefCell::new(DielectricV3::new(1.50))));
    let material_bubble: Option<Rc<RefCell<dyn Material>>> =
        Some(Rc::new(RefCell::new(DielectricV3::new(1.00 / 1.30))));
    let material_right: Option<Rc<RefCell<dyn Material>>> = Some(Rc::new(RefCell::new(
        MetalFuzz::new(&Color::from_slice([0.8, 0.6, 0.2]), 1.0),
    )));

    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([0.0, -100.5, -1.0]),
        100.0,
        material_ground,
    ))));
    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([0.0, 0.0, -1.2]),
        0.5,
        material_center,
    ))));
    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([-1.0, 0.0, -1.0]),
        0.5,
        material_left,
    ))));
    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([-1.0, 0.0, -1.0]),
        0.4,
        material_bubble,
    ))));
    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([1.0, 0.0, -1.0]),
        0.5,
        material_right,
    ))));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 45.0;
    camera.look_from = Point3::from_slice([-2.0, 2.0, 1.0]);
    camera.look_at = Point3::from_slice([0.0, 0.0, -1.0]);
    camera.v_up = Point3::from_slice([0.0, 1.0, 0.0]);

    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;

    let gamma = 0.50;
    camera.render_defocus(gamma, &world, &file_name)?;

    Ok(())
}
