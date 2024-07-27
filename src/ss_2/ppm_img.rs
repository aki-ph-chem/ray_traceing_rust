use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() < 2 {
        panic!("Error: invalid args");
    }

    let file_name = argv[1].clone();
    let (image_width, image_height) = (256, 256);
    let header = format!("P3\n{image_width} {image_height}\n255\n");
    let mut file = File::create(file_name)?;

    std::writeln!(&mut file, "{header}")?;
    for j in 0..image_height {
        for i in 0..image_height {
            let (r, g, b) = (
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            let (ir, ig, ib) = (
                (255.999 * r) as i32,
                (255.999 * g) as i32,
                (255.999 * b) as i32,
            );

            std::writeln!(&mut file, "{ir} {ig} {ib}")?;
        }
    }

    Ok(())
}
