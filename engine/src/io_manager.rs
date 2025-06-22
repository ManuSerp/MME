use std::fs;
use std::path::{Path, PathBuf};

use image::DynamicImage;
use anyhow::{Result, Context};

/// Represents a set of images loaded from disk.
pub struct ImageSet {
    pub images: Vec<DynamicImage>,
    pub filenames: Vec<PathBuf>,
}

/// Load all images in a given folder.
pub fn load_images<P: AsRef<Path>>(folder: P) -> Result<ImageSet> {
    let mut images = Vec::new();
    let mut filenames = Vec::new();

    for entry in fs::read_dir(&folder)
        .with_context(|| format!("Failed to read directory {:?}", folder.as_ref()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // Try to open image using `image` crate
            let img = image::open(&path)
                .with_context(|| format!("Failed to open image: {:?}", path))?;
            images.push(img);
            filenames.push(path);
        }
    }

    Ok(ImageSet { images, filenames })
}