use burn::{
    data::{dataloader::DataLoaderBuilder, dataset::vision::MnistDataset},
    module::AutodiffModule,
    nn::loss::{self, CrossEntropyLoss, CrossEntropyLossConfig, HuberLossConfig},
    optim::{AdamConfig, GradientsParams, Optimizer},
    prelude::*,
    record::{CompactRecorder, FullPrecisionSettings, NamedMpkFileRecorder},
    tensor::backend::AutodiffBackend,
    train::{
        ClassificationOutput, LearnerBuilder, RegressionOutput, TrainOutput, TrainStep, ValidStep,
        metric::{AccuracyMetric, LossMetric},
    },
};

use rand::prelude::*;

use crate::{
    data::{BatchedSimulationStep, DatasetGenerator, DatasetGeneratorConfig},
    model::{Model, ModelConfig, StateRepr},
};

#[derive(Debug, Config)]
pub struct TrainingConfig {
    pub model: ModelConfig,
    pub optimizer: AdamConfig,
    // #[config(default = 100)]
    #[config(default = 250)]
    pub num_epochs: usize,
    #[config(default = 64)]
    pub batch_size: usize,
    #[config(default = 4)]
    pub num_workers: usize,
    #[config(default = 42)]
    pub seed: u64,
    #[config(default = 1e-4)]
    pub learning_rate: f64,
}

fn create_artifact_dir(artifact_dir: &str) {
    std::fs::remove_dir_all(artifact_dir).ok();
    std::fs::create_dir_all(artifact_dir).ok();
}

pub(crate) fn run<B: AutodiffBackend>(
    artifact_dir: &str,
    config: TrainingConfig,
    device: B::Device,
    dgb: DatasetGeneratorConfig,
) {
    create_artifact_dir(artifact_dir);
    // Create the configuration.
    let config_model = ModelConfig::new(10, 1024);
    let config_optimizer = AdamConfig::new();
    let config = TrainingConfig::new(config_model, config_optimizer);
    let dataloader = dgb.build();

    B::seed(&device, config.seed);
    let mut rng = SmallRng::seed_from_u64(config.seed);

    // Create the model and optimizer.
    let mut model: Model<B> = config.model.init::<B>(&device);
    let mut optim = config.optimizer.init();

    // Iterate over our training and validation loop for X epochs.
    for epoch in 1..config.num_epochs + 1 {
        // Implement our training loop.
        for (iteration, batch) in dataloader
            .iter_with_model(&model.valid(), &device, &mut rng, true)
            .enumerate()
        {
            let loss = forward_pass(&device, &model, batch, dgb.rew_config.gamma_factor);

            println!(
                "[Train - Epoch {} - Iteration {}] Loss {:.3}",
                epoch,
                iteration,
                loss.clone().into_scalar(),
            );

            // Gradients for the current backward pass
            let grads = loss.backward();
            // Gradients linked to each parameter of the model.
            let grads = GradientsParams::from_grads(grads, &model);
            // Update the model using the optimizer.
            model = optim.step(config.learning_rate, model, grads);
        }

        // Get the model without autodiff.
        let model_valid = model.valid();

        // Implement our validation loop.
        for (iteration, batch) in dataloader
            .iter_with_model(&model_valid, &device, &mut rng, false)
            .enumerate()
        {
            let loss = forward_pass(&device, &model_valid, batch, dgb.rew_config.gamma_factor);

            println!(
                "[Valid - Epoch {} - Iteration {}] Loss {:.3}",
                epoch,
                iteration,
                loss.clone().into_scalar(),
            );
        }
    }

    // Load model in full precision from MessagePack file
    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
    model
        .save_file(format!("{}model.mpk", artifact_dir), &recorder)
        .expect("Should be able to load the model weights from the provided file"); //
}

fn forward_pass<B: Backend, C: Backend>(
    device: &B::Device,
    model: &Model<B>,
    BatchedSimulationStep {
        snapshot,
        direction,
        reward,
        next_state_qual,
    }: BatchedSimulationStep<C>,
    gamma_factor: f32,
) -> Tensor<B, 1, Float> {
    let attached: StateRepr<B> = StateRepr(Tensor::from_data(snapshot.to_data(), device));
    let direction: Tensor<B, 1, Int> = Tensor::from_data(direction.to_data(), device);
    let direction = direction.unsqueeze_dim(1);
    let reward = Tensor::from_data(reward.to_data(), device);
    let next_state_qual = Tensor::from_data(next_state_qual.to_data(), device);
    let out = model.forward(attached);
    let selected = out.gather(1, direction);
    let sel: Tensor<B, 1, Float> = selected.squeeze::<1>();
    loss::MseLoss::new().forward(
        sel,
        reward + next_state_qual.mul_scalar(gamma_factor),
        loss::Reduction::Auto,
    )
}
