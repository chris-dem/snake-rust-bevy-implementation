use burn::nn::{
    BatchNorm, BatchNormConfig, Dropout, DropoutConfig, Gelu, Linear, LinearConfig,
    PaddingConfig2d, Relu,
    conv::{Conv2d, Conv2dConfig},
    pool::{AdaptiveAvgPool2d, AdaptiveAvgPool2dConfig, MaxPool2d, MaxPool2dConfig},
};
use burn::prelude::*;

#[derive(Debug, Module)]
pub struct Model<B: Backend> {
    pool: MaxPool2d,
    dropout: Dropout,
    conv1: Conv2d<B>,
    conv1s: Conv2d<B>,
    conv2: Conv2d<B>,
    conv2s: Conv2d<B>,
    conv2ss: Conv2d<B>,
    lin1: Linear<B>,
    lin2: Linear<B>,
    act: Gelu,
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
            conv1: Conv2dConfig::new([1, 8], [3, 3])
                .with_padding(PaddingConfig2d::Same)
                .init(device),
            conv1s: Conv2dConfig::new([8, 8], [3, 3])
                .with_padding(PaddingConfig2d::Same)
                .init(device),
            conv2: Conv2dConfig::new([8, 16], [3, 3])
                .with_padding(PaddingConfig2d::Same)
                .init(device),
            conv2s: Conv2dConfig::new([16, 16], [3, 3])
                .with_padding(PaddingConfig2d::Same)
                .init(device),
            conv2ss: Conv2dConfig::new([16, 16], [3, 3])
                .with_padding(PaddingConfig2d::Same)
                .init(device),
            act: Gelu::new(),
            lin1: LinearConfig::new(16 * 8 * 10, self.hidden_size).init(device),
            lin2: LinearConfig::new(self.hidden_size, self.num_classes).init(device),
            dropout: DropoutConfig::new(self.dropout).init(),
            pool: MaxPool2dConfig::new([2, 2]).init(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateRepr<B: Backend>(pub Tensor<B, 4, Float>); // B R H [empty, snake head, snake body, food]

impl<B: Backend> Model<B> {
    /// #Shapes
    /// - Images [batch_size, height, width]
    /// - Output [batch_size, num_classes]
    pub fn forward(&self, state: StateRepr<B>) -> Tensor<B, 2> {
        let StateRepr(snapshot) = state;
        // Create a channel at the second dimension
        let [bdims, row, col, ch] = snapshot.dims();

        let x = snapshot.reshape([bdims, ch, row, col]);

        let x = self.conv1.forward(x); // 32 40
        let x = self.act.forward(x);
        let x = self.dropout.forward(x);
        let x = self.conv1s.forward(x);
        let x = self.act.forward(x);

        let x = self.pool.forward(x); // 16 20

        let x = self.conv2.forward(x);
        let x = self.act.forward(x);
        let x = self.dropout.forward(x);
        let x = self.conv2s.forward(x);
        let x = self.act.forward(x);

        let x = self.pool.forward(x); // 8 10

        let x = self.conv2s.forward(x);
        let x = self.act.forward(x);

        let x = x.reshape([bdims, 16 * 8 * 10]);
        let x = self.lin1.forward(x);
        let x = self.dropout.forward(x);
        let x = self.act.forward(x);
        self.lin2.forward(x)
    }
}
