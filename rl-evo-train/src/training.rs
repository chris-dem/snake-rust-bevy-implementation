use burn::{
    data::{dataloader::DataLoaderBuilder, dataset::vision::MnistDataset},
    nn::loss::{CrossEntropyLoss, CrossEntropyLossConfig, HuberLossConfig},
    optim::AdamConfig,
    prelude::*,
    record::CompactRecorder,
    tensor::backend::AutodiffBackend,
    train::{
        ClassificationOutput, LearnerBuilder, RegressionOutput, TrainOutput, TrainStep, ValidStep,
        metric::{AccuracyMetric, LossMetric},
    },
};

use crate::{
    data::{EpisodeSim, SnakeRLBatcher},
    model::{Model, ModelConfig},
};

struct GameInstanceModel;

impl<B: Backend> Model<B> {
    pub fn forward_classification(&self, batch: EpisodeSim<B>) -> RegressionOutput<B> {
        // let _ = self.forward(images);
        todo!("Calculate the forward propagations");
        todo!("calculate the loss (r_(t+1) - Q(s_t+1,a), current state)");
        // Just subtract, no MAE
        // needed

        // let loss = HuberLossConfig::new(0.5)
        //     .init()
        //     .forward(output.clone(), targets.clone());
    }
}

impl<B: AutodiffBackend> TrainStep<EpisodeSim<B>, RegressionOutput<B>> for Model<B> {
    fn step(&self, batch: EpisodeSim<B>) -> TrainOutput<RegressionOutput<B>> {
        let item = self.forward_classification(batch);
        todo!("Apply the TD learning algorithm");
        TrainOutput::new(self, item.loss.backward(), item)
    }
}

impl<B: Backend> ValidStep<EpisodeSim<B>, RegressionOutput<B>> for Model<B> {
    fn step(&self, batch: EpisodeSim<B>) -> RegressionOutput<B> {
        self.forward_classification(batch)
    }
}

#[derive(Debug, Config)]
pub struct TrainingConfig {
    pub model: ModelConfig,
    pub optimizer: AdamConfig,
    #[config(default = 10)]
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

pub fn run<B: AutodiffBackend>(artifact_dir: &str, config: TrainingConfig, device: B::Device) {
    // Create the configuration.
    let config_model = ModelConfig::new(10, 1024);
    let config_optimizer = AdamConfig::new();
    let config = ModelConfig::new(config_model, config_optimizer);

    B::seed(&device, config.seed);

    // Create the model and optimizer.
    let mut model = config.model.init::<B>(&device);
    let mut optim = config.optimizer.init();

    let batcher = SnakeRLBatcher::default();
    // Iterate over our training and validation loop for X epochs.
    for epoch in 1..config.num_epochs + 1 {
        // Implement our training loop.
        for (iteration, batch) in dataloader_train.iter().enumerate() {
            let output = model.forward(batch.images);
            let loss = CrossEntropyLoss::new(None, &output.device())
                .forward(output.clone(), batch.targets.clone());
            let accuracy = accuracy(output, batch.targets);

            println!(
                "[Train - Epoch {} - Iteration {}] Loss {:.3} | Accuracy {:.3} %",
                epoch,
                iteration,
                loss.clone().into_scalar(),
                accuracy,
            );

            // Gradients for the current backward pass
            let grads = loss.backward();
            // Gradients linked to each parameter of the model.
            let grads = GradientsParams::from_grads(grads, &model);
            // Update the model using the optimizer.
            model = optim.step(config.lr, model, grads);
        }

        // Get the model without autodiff.
        let model_valid = model.valid();

        // Implement our validation loop.
        for (iteration, batch) in dataloader_test.iter().enumerate() {
            let output = model_valid.forward(batch.images);
            let loss = CrossEntropyLoss::new(None, &output.device())
                .forward(output.clone(), batch.targets.clone());
            let accuracy = accuracy(output, batch.targets);

            println!(
                "[Valid - Epoch {} - Iteration {}] Loss {} | Accuracy {}",
                epoch,
                iteration,
                loss.clone().into_scalar(),
                accuracy,
            );
        }
    }
    //
}
