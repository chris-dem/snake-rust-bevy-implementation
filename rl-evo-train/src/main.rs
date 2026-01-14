#![recursion_limit = "256"]
pub mod data;
pub mod model;
pub mod training;
use crate::{data::DatasetGeneratorConfig, model::ModelConfig, training::TrainingConfig};
use anyhow::Result as ARes;
use burn::{
    backend::{Autodiff, Wgpu, wgpu::WgpuDevice},
    optim::AdamConfig,
};

fn main() -> ARes<()> {
    type MyBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device = WgpuDevice::default();
    let artifact_dir = "/tmp/burn-tutorial/";
    let dataset_cfg: DatasetGeneratorConfig =
        serde_json::from_str(&std::fs::read_to_string("./config.json")?)?;

    crate::training::run::<MyAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device.clone(),
        dataset_cfg,
    );
    Ok(())
}
