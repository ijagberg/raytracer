use std::env;
use std::fs::File;
use std::io::Write;
use std::iter;
use std::path::Path;
extern crate raytracer;
use raytracer::vec3::Vec3;

fn main() {
    match try_main() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

fn try_main() -> Result<(), Box<std::error::Error>> {
    println!("Writing file...");

    let test_file = {
        let mut current_dir = env::current_dir()?;
        current_dir.push("test_file.ppm");
        current_dir
    };

    let rows = 100_u64;
    let cols = 200_u64;
    let ppm_text = iter::once("P3".into())
        .chain(iter::once(format!("{} {}", cols, rows)))
        .chain(iter::once("255".into()))
        .chain((0..rows).flat_map(|row| {
            (0..cols).map(move |col| {
                let vec = Vec3::new(row as f64 / rows as f64, col as f64 / cols as f64, 0.2);
                format!(
                    "{} {} {}",
                    (vec.x() * 255.9) as usize,
                    (vec.y() * 255.9) as usize,
                    (vec.z() * 255.9) as usize
                )
            })
        }))
        .collect::<Vec<String>>()
        .join("\n");

    write_ppm(test_file, ppm_text.as_ref())
}

fn write_ppm<P: AsRef<Path>>(file: P, text: &str) -> Result<(), Box<std::error::Error>> {
    let mut file = File::create(file)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
