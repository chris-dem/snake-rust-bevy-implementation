use burn::{
    data::{
        dataloader::{DataLoader, batcher::*},
        dataset::vision::MnistItem,
    },
    prelude::*,
};

#[derive(Clone, Default)]
pub struct SnakeRLBatcher;

#[derive(Clone, Debug)]
pub struct EpisodeSim<B: Backend> {
    pub image_states: Tensor<B, 3>,
}

type ItemInput = usize;

impl<B: Backend> Batcher<B, ItemInput, EpisodeSim<B>> for SnakeRLBatcher {
    fn batch(&self, items: Vec<ItemInput>, device: &<B as Backend>::Device) -> EpisodeSim<B> {
        todo!()
    }
}
