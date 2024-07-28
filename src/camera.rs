use crate::color::{write_color, Color};
use crate::hittable::HitRecord;
use crate::hittable::HittableV2;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utl;
use crate::vec3::{Point3, Vec3};
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 0.0,
            image_width: 0,
            image_height: 0,
            center: Point3::new(),
            pixel00_loc: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
        }
    }

    pub fn render<T: HittableV2>(
        &mut self,
        world: &T,
        file_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.initialize();

        let mut file = File::create(file_name)?;
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        std::writeln!(&mut file, "{header}")?;
        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc.clone()
                    + (i as f64 * self.pixel_delta_u.clone())
                    + (j as f64 * self.pixel_delta_v.clone());
                let ray_direction = pixel_center - self.center.clone();
                let ray = Ray::from_origin_dir(&self.center, &ray_direction);

                let pixel_color = Self::ray_color(&ray, world);
                write_color(&mut file, &pixel_color)?;
            }
        }
        eprintln!("\rDone.   ");

        Ok(())
    }

    pub fn initialize(&mut self) {
        // calculate the image height (Its ensure that it's at leat 1)
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        // camera
        self.center = Point3::from_slice([0.0, 0.0, 0.0]);
        let focal_lenth = 1.0;
        let viewport_height = 2.0;
        let viweport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // calculate the vector across the horizontal and down the vertical viewport edge.
        let viewport_u = Vec3::from_slice([viweport_width, 0.0, 0.0]);
        let viewport_v = Vec3::from_slice([0.0, -viewport_height, 0.0]);

        // calculate the horizontal. and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u.clone() / self.image_width as f64;
        self.pixel_delta_v = viewport_v.clone() / self.image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left = self.center.clone()
            - Vec3::from_slice([0.0, 0.0, focal_lenth])
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc =
            viewport_upper_left + 0.5 * (self.pixel_delta_u.clone() + self.pixel_delta_v.clone());
    }

    pub fn ray_color<T: HittableV2>(ray: &Ray, world: &T) -> Color {
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
}
