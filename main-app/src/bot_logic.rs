use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy_rand::{global::GlobalRng, prelude::ChaCha8Rng};
use burn::{
    backend::NdArray,
    prelude::*,
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
};
use rand::{SeedableRng, rngs::SmallRng};
use rand_chacha::rand_core::RngCore;
use rl_evo_train::{
    data::PlayerModel,
    model::{Model, ModelConfig, StateRepr},
};
use snake_api_lib::simulator::PlayerTrait;

use crate::game_logic::{GameState, step_snake};

pub struct BotAgent;

#[derive(Resource)]
pub struct Agent(pub Arc<Mutex<Model<MyBackend>>>);

type MyBackend = NdArray<f32, i32>;

fn load_file() -> Agent {
    // Load model in full precision from MessagePack file
    // let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
    //
    // let device = Default::default();
    // let model: Model<MyBackend> = ModelConfig::new(4, 1024)
    //     .init(&device)
    //     .load_file("/tmp/burn-tutorial/model.mpk", &recorder, &device)
    //     .expect("Should be able to load the model weights from the provided file");
    // Agent(Arc::new(Mutex::new(model)))
    todo!("Fix")
}

pub fn set_dir_agent(
    agent: Res<Agent>,
    mut game_state: ResMut<GameState>,
    mut rng: Single<&mut ChaCha8Rng, With<GlobalRng>>,
) {
    // let device = Default::default();
    // let ag = agent.0.lock().expect("should be lockable");
    // let player = PlayerModel {
    //     model: &ag,
    //     active_mode: false,
    //     eps: 0.,
    //     device: &device,
    // };
    //
    // let dir = player.choose_dir(&game_state.0, &mut SmallRng::seed_from_u64(rng.next_u64()));
    // game_state.0.update_direction(dir);
}

impl Plugin for BotAgent {
    fn build(&self, app: &mut App) {
        // todo
        // let agent = load_file();
        // app.insert_resource(agent);
    }
}
