use burn::nn::{
    Dropout, DropoutConfig, Linear, LinearConfig, Relu,
    conv::{Conv2d, Conv2dConfig},
    pool::{AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig},
};
use burn::prelude::*;

#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    conv1: Conv2d<B>,
    conv2: Conv2d<B>,
    pool: AdaptiveAvgPool2d,
    dropout: Dropout,
    lin1: Linear<B>,
    lin2: Linear<B>,
    act: Relu,
}

#[derive(Debug, Config)]
pub struct ModelConfig {
    num_classes: usize,
    hidden_size: usize,
    #[config(default = "0.5")]
    dropout: f64,
}

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model {
            conv1: Conv2dConfig::new([1, 8], [3, 3]).init(device),
            conv2: Conv2dConfig::new([8, 16], [3, 3]).init(device),
            pool: AdaptiveAvgPool2dConfig::new([8, 8]).init(),
            act: Relu::new(),
            lin1: LinearConfig::new(16 * 8 * 8, self.hidden_size).init(device),
            lin2: LinearConfig::new(self.hidden_size, self.num_classes).init(device),
            dropout: DropoutConfig::new(self.dropout).init(),
        }
    }
}

impl<B: Backend> Model<B> {
    /// #Shapes
    /// - Images [batch_size, height, width]
    /// - Output [batch_size, num_classes]
    pub fn forward(&self, images: Tensor<B, 3>) -> Tensor<B, 2> {
        let [batch_size, height, width] = images.dims();
        // Create a channel at the second dimension
        let x = images.reshape([batch_size, 1, height, width]);

        let x = self.conv1.forward(x);
        let x = self.act.forward(x);
        let x = self.dropout.forward(x);
        let x = self.conv2.forward(x);
        let x = self.act.forward(x);

        let x = self.pool.forward(x);
        let x = x.reshape([batch_size, 16 * 8 * 8]);
        let x = self.lin1.forward(x);
        let x = self.dropout.forward(x);
        let x = self.act.forward(x);

        self.lin2.forward(x)
    }
}
