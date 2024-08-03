use ray_tracing_rust::camera::Camera;
use ray_tracing_rust::color::Color;
use ray_tracing_rust::hittable_list::HittableList;
use ray_tracing_rust::material::{DielectricV3, Lambertian, Material, MetalFuzz};
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
    let material_ground: Option<Rc<RefCell<dyn Material>>> = Some(Rc::new(RefCell::new(
        Lambertian::new(&Color::from_slice([0.8, 0.8, 0.0])),
    )));
    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([0.0, -100.5, -1.0]),
        100.0,
        material_ground,
    ))));

    let mut random = utl::Random::new();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random.random_f64();
            let center = Point3::from_slice([
                a as f64 + 0.9 * random.random_f64(),
                0.2,
                b as f64 + 0.9 * random.random_f64(),
            ]);

            if (center.clone() - Point3::from_slice([4.0, 0.2, 0.0])).norm() > 0.9 {
                let mut material_sphere: Option<Rc<RefCell<dyn Material>>> = None;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    material_sphere = Some(Rc::new(RefCell::new(Lambertian::new(&albedo))));
                    world.add(Rc::new(RefCell::new(SphereMat::new(
                        &center,
                        0.2,
                        material_sphere,
                    ))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_by_range(0.5, 1.0);
                    let fuzz = random.random_f64_range(0.0, 0.5);
                    material_sphere = Some(Rc::new(RefCell::new(MetalFuzz::new(&albedo, fuzz))));

                    world.add(Rc::new(RefCell::new(SphereMat::new(
                        &center,
                        0.2,
                        material_sphere,
                    ))));
                } else {
                    // glass
                    material_sphere = Some(Rc::new(RefCell::new(DielectricV3::new(1.5))));

                    world.add(Rc::new(RefCell::new(SphereMat::new(
                        &center,
                        0.2,
                        material_sphere,
                    ))));
                }
            }
        }
    }

    let material_1: Option<Rc<RefCell<dyn Material>>> =
        Some(Rc::new(RefCell::new(DielectricV3::new(1.5))));
    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([0.0, 1.0, 0.0]),
        1.0,
        material_1,
    ))));

    let material_2: Option<Rc<RefCell<dyn Material>>> = Some(Rc::new(RefCell::new(
        Lambertian::new(&Color::from_slice([0.4, 0.2, 0.1])),
    )));
    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([-4.0, 1.0, 0.0]),
        1.0,
        material_2,
    ))));

    let material_3: Option<Rc<RefCell<dyn Material>>> = Some(Rc::new(RefCell::new(
        MetalFuzz::new(&Color::from_slice([0.7, 0.6, 0.5]), 0.0),
    )));
    world.add(Rc::new(RefCell::new(SphereMat::new(
        &Point3::from_slice([4.0, 1.0, 0.0]),
        1.0,
        material_3,
    ))));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.look_from = Point3::from_slice([13.0, 2.0, 3.0]);
    camera.look_at = Point3::new();
    camera.v_up = Point3::from_slice([0.0, 1.0, 0.0]);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let gamma = 0.50;
    camera.render_defocus(gamma, &world, &file_name)?;

    Ok(())
}
