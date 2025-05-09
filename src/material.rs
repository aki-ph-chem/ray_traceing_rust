use crate::color::Color;
use crate::hittable_material::HitRecordMat;
use crate::ray::Ray;
use crate::utl;
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
        r_in: &Ray,
        rec: &HitRecordMat,
        attennuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal.clone() + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        *scattered = Ray::from_origin_dir_tm(&rec.p, &scatter_direction, r_in.time());
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
        *scattered = Ray::from_origin_dir_tm(&rec.p, &reflected, r_in.time());
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
        *scattered = Ray::from_origin_dir_tm(&rec.p, &reflected, r_in.time());
        *attennuation = self.albedo.clone();

        scattered.direction().dot(&rec.normal) > 0.0
    }
}

/// not consider total internal reflection
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

/// consider total internal reflection
pub struct DielectricV2 {
    refraction_index: f64,
}

impl DielectricV2 {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for DielectricV2 {
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
        let cos_theta = (-unit_direction.dot(&rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };
        *scattered = Ray::from_origin_dir(&rec.p, &direction);

        true
    }
}

/// with Schlick Approximation: dependencies of refractive index by angle
pub struct DielectricV3 {
    refraction_index: f64,
}

impl DielectricV3 {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r_0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);

        r_0 + (1.0 - r_0) * (1.0 - cosine).powi(5)
    }
}

impl Material for DielectricV3 {
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
        let cos_theta = (-unit_direction.dot(&rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > utl::random_f64() {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };
        *scattered = Ray::from_origin_dir_tm(&rec.p, &direction, r_in.time());

        true
    }
}
