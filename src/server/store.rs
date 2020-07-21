use crate::image::Image;
use std::io::Write;

pub fn store(image: &Image, dir: &std::path::Path) -> Result<(), std::io::Error> {
    if !dir.exists() {
        std::fs::create_dir(dir)?;
    }

    let file_path = dir.join(image.name())
        .with_extension(image.extension());

    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&file_path)?
        .write(image.data())?;
    Ok(())
}
