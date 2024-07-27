use crate::vec3::Vec3;
pub type Color = Vec3;

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
