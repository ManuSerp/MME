use engine::orchestrator::{PipelineConfig, run_pipeline};

fn main() {
    let cfg = PipelineConfig {
        input_dir: "data/images".into(), 
    };

    if let Err(e) = run_pipeline(&cfg) {
        eprintln!("Error: {:#}", e);
    }
}