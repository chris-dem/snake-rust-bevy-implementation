
// Two Baseline models
// 1. Single line location (easy game)
// 1. Grid with key and door location (harder game)

#[derive(Debug)]
pub struct ToyModel;

#[derive(Debug)]
pub struct ToyProblem;

#[derive(Debug)]
pub struct QStarTrainer {
    model: ToyModel,
}

impl QStarTrainer {
    fn new() -> Self {
        Self { model: ToyModel }
    }
    fn train() {}
}

fn main() {
    println!("Hello, world!");
}
