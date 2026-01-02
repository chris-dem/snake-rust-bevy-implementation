use itertools::Itertools;
use rand::{prelude::*, seq::SliceRandom};
use snake_api_lib::{
    GameAPI,
    common::{Direction, DirectionIter},
};
use strum::IntoEnumIterator;

pub trait PlayerTrait {
    fn select_direction(&mut self, game: &GameAPI) -> Direction;
}

pub struct RandomPlayer(Box<dyn RngCore>);

impl PlayerTrait for RandomPlayer {
    fn select_direction(&mut self, _: &GameAPI) -> Direction {
        let rng = self.0.as_mut();
        *Direction::iter()
            .collect_vec()
            .choose(rng)
            .expect("Should have at least 1 defined direction")
    }
}


