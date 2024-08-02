use crate::color::Color;
use crate::hittable_material::HitRecordMat;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecordMat,
        _attennuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Self {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecordMat,
        attennuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal.clone() + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        *scattered = Ray::from_origin_dir(&rec.p, &scatter_direction);
        *attennuation = self.albedo.clone();
        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: &Color) -> Self {
        Self {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecordMat,
        attennuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction(), &rec.normal);
        *scattered = Ray::from_origin_dir(&rec.p, &reflected);
        *attennuation = self.albedo.clone();

        true
    }
}

pub struct MetalFuzz {
    albedo: Color,
    fuzz: f64,
}

impl MetalFuzz {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self {
            albedo: albedo.clone(),
            fuzz,
        }
    }
}

impl Material for MetalFuzz {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecordMat,
        attennuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vec3::reflect(r_in.direction(), &rec.normal);
        reflected.normalize();
        reflected += self.fuzz * Vec3::random_unit_vector();
        *scattered = Ray::from_origin_dir(&rec.p, &reflected);
        *attennuation = self.albedo.clone();

        scattered.direction().dot(&rec.normal) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecordMat,
        attennuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attennuation = Color::from_slice([1.0, 1.0, 1.0]);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::new_unit_vec(r_in.direction().clone());
        let refracted = Vec3::refract(&unit_direction, &rec.normal, ri);
        *scattered = Ray::from_origin_dir(&rec.p, &refracted);

        true
    }
}
