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
        let reflected = Vec3::refrect(r_in.direction(), &rec.normal);
        *scattered = Ray::from_origin_dir(&rec.p, &reflected);
        *attennuation = self.albedo.clone();

        true
    }
}
