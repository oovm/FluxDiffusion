use std::path::PathBuf;

pub struct MigrationPlan {
    // flux dev
    flux: FluxTensors,
    base_weight: f32,
    // safe tensors
    merges: Vec<(ClipTensors, f32)>,
    // flux dev bf 16
    output: PathBuf,
}