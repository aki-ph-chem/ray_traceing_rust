use ray_tracing_rust::color::{write_color, Color};
use ray_tracing_rust::interval::Interval;
use ray_tracing_rust::ray::Ray;
use ray_tracing_rust::sphere::Sphere;
use ray_tracing_rust::utl;
use ray_tracing_rust::vec3::{Point3, Vec3};
use ray_tracing_rust::{hittable::HitRecord, hittable::HittableV2, hittable_list::HittableList};
use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

fn ray_color<T: HittableV2>(ray: &Ray, world: &T) -> Color {
    let mut rec = HitRecord::new();
    if world.hit_v2(
        &ray,
        Interval::new_by_value(0.0, utl::constans::INFINITY),
        &mut rec,
    ) {
        return 0.5 * (rec.normal.clone() + Color::from_slice([1.0, 1.0, 1.0]));
    }

    let unit_direction = Vec3::new_unit_vec(ray.direction().clone());
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::from_slice([1.0, 1.0, 1.0]) + a * Color::from_slice([0.5, 0.7, 1.0])
}

fn main() -> Result<(), Box<dyn Error>> {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() < 2 {
        panic!("Error: invalid args");
    }
    let file_name = argv[1].clone();

    // image
    let aspect_ration = 16.0 / 9.0;
    let image_width = 400;

    // calculate the image height (Its ensure that it's at leat 1)
    let image_height = (image_width as f64 / aspect_ration) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // world
    let mut world = HittableList::new();
    world.add(Rc::new(RefCell::new(Sphere::new(
        &Point3::from_slice([0.0, 0.0, -1.0]),
        0.5,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        &Point3::from_slice([0.0, -100.5, -1.0]),
        100.0,
    ))));

    // camera
    let focal_lenth = 1.0;
    let viewport_height = 2.0;
    let viweport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::from_slice([0.0, 0.0, 0.0]);

    // calculate the vector across the horizontal and down the vertical viewport edge.
    let viewport_u = Vec3::from_slice([viweport_width, 0.0, 0.0]);
    let viewport_v = Vec3::from_slice([0.0, -viewport_height, 0.0]);

    // calculate the horizontal. and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u.clone() / image_width as f64;
    let pixel_delta_v = viewport_v.clone() / image_height as f64;

    // calculate the location of the upper left pixel
    let viewport_upper_left = camera_center.clone()
        - Vec3::from_slice([0.0, 0.0, focal_lenth])
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u.clone() + pixel_delta_v.clone());

    // render
    let mut file = File::create(file_name)?;
    let header = format!("P3\n{image_width} {image_height}\n255\n");
    std::writeln!(&mut file, "{header}")?;
    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel100_loc.clone()
                + (i as f64 * pixel_delta_u.clone())
                + (j as f64 * pixel_delta_v.clone());
            let ray_direction = pixel_center - camera_center.clone();
            let ray = Ray::from_origin_dir(&camera_center, &ray_direction);

            let pixel_color = ray_color(&ray, &world);
            write_color(&mut file, &pixel_color)?;
        }
    }
    eprintln!("\rDone.   ");

    Ok(())
}
