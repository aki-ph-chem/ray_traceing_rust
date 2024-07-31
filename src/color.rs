use crate::interval::Interval;
use crate::vec3::Vec3;
pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64, gamma: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.powf(gamma);
    }

    0.0
}

pub fn write_color<T: std::io::Write>(
    out: &mut T,
    pixel_color: &Color,
) -> Result<(), Box<dyn std::error::Error>> {
    let (r, g, b) = (pixel_color.x(), pixel_color.y(), pixel_color.z());
    let (r_byte, g_byte, b_byte) = (
        (255.999 * r) as i32,
        (255.999 * g) as i32,
        (255.999 * b) as i32,
    );

    std::writeln!(out, "{r_byte} {g_byte} {b_byte}")?;
    Ok(())
}

pub fn write_color_v2<T: std::io::Write>(
    out: &mut T,
    pixel_color: &Color,
) -> Result<(), Box<dyn std::error::Error>> {
    let (r, g, b) = (pixel_color.x(), pixel_color.y(), pixel_color.z());
    let intensity = Interval::new_by_value(0.00, 0.999);
    let (r_byte, g_byte, b_byte) = (
        (256.000 * intensity.clamp(r)) as i32,
        (256.000 * intensity.clamp(g)) as i32,
        (256.000 * intensity.clamp(b)) as i32,
    );

    std::writeln!(out, "{r_byte} {g_byte} {b_byte}")?;
    Ok(())
}

pub fn write_color_gamma<T: std::io::Write>(
    gamma: f64,
    out: &mut T,
    pixel_color: &Color,
) -> Result<(), Box<dyn std::error::Error>> {
    let (r, g, b) = (pixel_color.x(), pixel_color.y(), pixel_color.z());
    let (r, g, b) = (
        linear_to_gamma(r, gamma),
        linear_to_gamma(g, gamma),
        linear_to_gamma(b, gamma),
    );

    let intensity = Interval::new_by_value(0.00, 0.999);
    let (r_byte, g_byte, b_byte) = (
        (256.000 * intensity.clamp(r)) as i32,
        (256.000 * intensity.clamp(g)) as i32,
        (256.000 * intensity.clamp(b)) as i32,
    );

    std::writeln!(out, "{r_byte} {g_byte} {b_byte}")?;
    Ok(())
}
