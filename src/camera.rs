use crate::color::{write_color, write_color_gamma, Color};
use crate::hittable::HitRecord;
use crate::hittable::HittableV2;
use crate::hittable_material::{HitRecordMat, HittableMat};
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
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub v_up: Point3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pixel_samples_scale: f64,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    random: utl::Random,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            look_from: Point3::new(),
            look_at: Point3::from_slice([0.0, 0.0, -1.0]),
            v_up: Point3::from_slice([0.0, 1.0, 0.0]),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            pixel_samples_scale: 0.0,
            image_height: 0,
            center: Point3::new(),
            pixel00_loc: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
            random: utl::Random::new(),
            u: Vec3::new(),
            v: Vec3::new(),
            w: Vec3::new(),
            defocus_disk_u: Vec3::new(),
            defocus_disk_v: Vec3::new(),
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

    pub fn render_v2<T: HittableV2>(
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
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color(&mut file, &pixel_color)?;
                pixel_color /= self.pixel_samples_scale;
            }
        }
        eprintln!("\rDone.   ");

        Ok(())
    }

    pub fn render_diffuse<T: HittableV2>(
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
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color_diffuse(&ray, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color(&mut file, &pixel_color)?;
                pixel_color /= self.pixel_samples_scale;
            }
        }
        eprintln!("\rDone.   ");

        Ok(())
    }

    pub fn render_diffuse_max_depth<T: HittableV2>(
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
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color_diffuse_max_depth(&ray, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color(&mut file, &pixel_color)?;
                pixel_color /= self.pixel_samples_scale;
            }
        }
        eprintln!("\rDone.   ");

        Ok(())
    }

    pub fn render_lambertian<T: HittableV2>(
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
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color_lambertian(&ray, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color(&mut file, &pixel_color)?;
                pixel_color /= self.pixel_samples_scale;
            }
        }
        eprintln!("\rDone.   ");

        Ok(())
    }

    pub fn render_gamma<T: HittableV2>(
        &mut self,
        gamma: f64,
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
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color_lambertian(&ray, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color_gamma(gamma, &mut file, &pixel_color)?;
                pixel_color /= self.pixel_samples_scale;
            }
        }
        eprintln!("\rDone.   ");

        Ok(())
    }

    pub fn render_material<T: HittableMat>(
        &mut self,
        gamma: f64,
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
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color_material(&ray, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color_gamma(gamma, &mut file, &pixel_color)?;
                pixel_color /= self.pixel_samples_scale;
            }
        }
        eprintln!("\rDone.   ");

        Ok(())
    }

    pub fn render_view<T: HittableMat>(
        &mut self,
        gamma: f64,
        world: &T,
        file_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.initialize_view();

        let mut file = File::create(file_name)?;
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        std::writeln!(&mut file, "{header}")?;
        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color_material(&ray, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color_gamma(gamma, &mut file, &pixel_color)?;
                pixel_color /= self.pixel_samples_scale;
            }
        }
        eprintln!("\rDone.   ");

        Ok(())
    }

    pub fn render_defocus<T: HittableMat>(
        &mut self,
        gamma: f64,
        world: &T,
        file_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.initialize_defocus();

        let mut file = File::create(file_name)?;
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        std::writeln!(&mut file, "{header}")?;
        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray_defocus(i, j);
                    pixel_color += Self::ray_color_material(&ray, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color_gamma(gamma, &mut file, &pixel_color)?;
                pixel_color /= self.pixel_samples_scale;
            }
        }
        eprintln!("\rDone.   ");

        Ok(())
    }

    pub fn render_motion_blur<T: HittableMat>(
        &mut self,
        gamma: f64,
        world: &T,
        file_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.initialize_defocus();

        let mut file = File::create(file_name)?;
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        std::writeln!(&mut file, "{header}")?;
        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray_motion_blur(i, j);
                    pixel_color += Self::ray_color_material(&ray, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale;
                write_color_gamma(gamma, &mut file, &pixel_color)?;
                pixel_color /= self.pixel_samples_scale;
            }
        }
        eprintln!("\rDone.   ");

        Ok(())
    }

    fn initialize(&mut self) {
        // calculate the image height (Its ensure that it's at leat 1)
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        // camera
        self.center = Point3::from_slice([0.0, 0.0, 0.0]);
        let focal_lenth = 1.0;
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_lenth;
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

    fn initialize_view(&mut self) {
        // calculate the image height (Its ensure that it's at leat 1)
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        // camera
        self.center = self.look_from.clone();
        let focal_lenth = (self.look_from.clone() - self.look_at.clone()).norm();
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_lenth;
        let viweport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = Vec3::new_unit_vec(self.look_from.clone() - self.look_at.clone());
        self.u = Vec3::new_unit_vec(self.v_up.cross(&self.w));
        self.v = self.w.cross(&self.u);

        // calculate the vector across the horizontal and down the vertical viewport edge.
        let viewport_u = viweport_width * self.u.clone();
        let viewport_v = -viewport_height * self.v.clone();

        // calculate the horizontal. and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u.clone() / self.image_width as f64;
        self.pixel_delta_v = viewport_v.clone() / self.image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left = self.center.clone()
            - (focal_lenth * self.w.clone())
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc =
            viewport_upper_left + 0.5 * (self.pixel_delta_u.clone() + self.pixel_delta_v.clone());
    }

    fn initialize_defocus(&mut self) {
        // calculate the image height (Its ensure that it's at leat 1)
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        // camera
        self.center = self.look_from.clone();
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viweport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = Vec3::new_unit_vec(self.look_from.clone() - self.look_at.clone());
        self.u = Vec3::new_unit_vec(self.v_up.cross(&self.w));
        self.v = self.w.cross(&self.u);

        // calculate the vector across the horizontal and down the vertical viewport edge.
        let viewport_u = viweport_width * self.u.clone();
        let viewport_v = -viewport_height * self.v.clone();

        // calculate the horizontal. and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u.clone() / self.image_width as f64;
        self.pixel_delta_v = viewport_v.clone() / self.image_height as f64;

        // calculate the location of the upper left pixel
        let viewport_upper_left = self.center.clone()
            - (self.focus_dist * self.w.clone())
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc =
            viewport_upper_left + 0.5 * (self.pixel_delta_u.clone() + self.pixel_delta_v.clone());

        // calculate the camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * (self.defocus_angle.to_radians() / 2.0).tan();
        self.defocus_disk_u = defocus_radius * self.u.clone();
        self.defocus_disk_v = defocus_radius * self.v.clone();
    }

    fn get_ray(&mut self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc.clone()
            + (i as f64 + offset.x()) * self.pixel_delta_u.clone()
            + (j as f64 + offset.y()) * self.pixel_delta_v.clone();
        let ray_origin = self.center.clone();
        let ray_direction = pixel_sample - ray_origin.clone();

        Ray::from_origin_dir(&ray_origin, &ray_direction)
    }

    fn get_ray_defocus(&mut self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc.clone()
            + (i as f64 + offset.x()) * self.pixel_delta_u.clone()
            + (j as f64 + offset.y()) * self.pixel_delta_v.clone();
        let ray_origin = if self.defocus_angle < 0.0 {
            self.center.clone()
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin.clone();

        Ray::from_origin_dir(&ray_origin, &ray_direction)
    }

    fn get_ray_motion_blur(&mut self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc.clone()
            + (i as f64 + offset.x()) * self.pixel_delta_u.clone()
            + (j as f64 + offset.y()) * self.pixel_delta_v.clone();
        let ray_origin = if self.defocus_angle < 0.0 {
            self.center.clone()
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin.clone();
        let ray_time = utl::random_f64();

        Ray::from_origin_dir_tm(&ray_origin, &ray_direction, ray_time)
    }

    fn sample_square(&mut self) -> Vec3 {
        Vec3::from_slice([
            self.random.random_f64() - 0.5,
            self.random.random_f64() - 0.5,
            0.0,
        ])
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();

        self.center.clone()
            + (p[0] * self.defocus_disk_u.clone())
            + (p[1] * self.defocus_disk_v.clone())
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

    pub fn ray_color_diffuse<T: HittableV2>(ray: &Ray, world: &T) -> Color {
        let mut rec = HitRecord::new();
        if world.hit_v2(
            &ray,
            Interval::new_by_value(0.0, utl::constans::INFINITY),
            &mut rec,
        ) {
            let direction = Vec3::random_on_hemisphere(&rec.normal);
            return 0.5 * Self::ray_color_diffuse(&Ray::from_origin_dir(&rec.p, &direction), world);
        }

        let unit_direction = Vec3::new_unit_vec(ray.direction().clone());
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::from_slice([1.0, 1.0, 1.0]) + a * Color::from_slice([0.5, 0.7, 1.0])
    }

    pub fn ray_color_diffuse_max_depth<T: HittableV2>(ray: &Ray, depth: i32, world: &T) -> Color {
        if depth <= 0 {
            return Color::from_slice([0.0, 0.0, 0.0]);
        }

        let mut rec = HitRecord::new();
        if world.hit_v2(
            &ray,
            Interval::new_by_value(0.001, utl::constans::INFINITY),
            &mut rec,
        ) {
            let direction = Vec3::random_on_hemisphere(&rec.normal);
            return 0.5
                * Self::ray_color_diffuse_max_depth(
                    &Ray::from_origin_dir(&rec.p, &direction),
                    depth - 1,
                    world,
                );
        }

        let unit_direction = Vec3::new_unit_vec(ray.direction().clone());
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::from_slice([1.0, 1.0, 1.0]) + a * Color::from_slice([0.5, 0.7, 1.0])
    }

    pub fn ray_color_lambertian<T: HittableV2>(ray: &Ray, depth: i32, world: &T) -> Color {
        if depth <= 0 {
            return Color::from_slice([0.0, 0.0, 0.0]);
        }

        let mut rec = HitRecord::new();
        if world.hit_v2(
            &ray,
            Interval::new_by_value(0.001, utl::constans::INFINITY),
            &mut rec,
        ) {
            let direction = rec.normal.clone() + Vec3::random_unit_vector();
            return 0.5
                * Self::ray_color_diffuse_max_depth(
                    &Ray::from_origin_dir(&rec.p, &direction),
                    depth - 1,
                    world,
                );
        }

        let unit_direction = Vec3::new_unit_vec(ray.direction().clone());
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::from_slice([1.0, 1.0, 1.0]) + a * Color::from_slice([0.5, 0.7, 1.0])
    }

    pub fn ray_color_material<T: HittableMat>(ray: &Ray, depth: i32, world: &T) -> Color {
        if depth <= 0 {
            return Color::from_slice([0.0, 0.0, 0.0]);
        }

        let mut rec = HitRecordMat::new();
        if world.hit_mat(
            &ray,
            Interval::new_by_value(0.001, utl::constans::INFINITY),
            &mut rec,
        ) {
            let mut scatterd = Ray::new();
            let mut attenuation = Color::new();
            if rec.mat.clone().unwrap().as_ref().borrow().scatter(
                &ray,
                &rec,
                &mut attenuation,
                &mut scatterd,
            ) {
                return attenuation * Self::ray_color_material(&scatterd, depth - 1, world);
            }
            return Color::new();
        }

        let unit_direction = Vec3::new_unit_vec(ray.direction().clone());
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::from_slice([1.0, 1.0, 1.0]) + a * Color::from_slice([0.5, 0.7, 1.0])
    }
}
