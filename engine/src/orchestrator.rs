use anyhow::Result;

use crate::io_manager;

/// Basic config for the pipeline.
pub struct PipelineConfig {
    pub input_dir: String,
}

/// Entry point: runs the minimal pipeline.
pub fn run_pipeline(cfg: &PipelineConfig) -> Result<()> {
    println!("=== Running Pipeline ===");
    println!("Input directory: {}", cfg.input_dir);

    // Load images
    let image_set = io_manager::load_images(&cfg.input_dir)?;
    println!("Loaded {} images.", image_set.images.len());

    // Stub for SFM step
    println!("Running SFM... [stub]");

    // Stub for LOD step
    println!("Generating LOD... [stub]");

    // Stub for Export step
    println!("Exporting results... [stub]");

    println!("=== Pipeline finished ===");
    Ok(())
}