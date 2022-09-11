use bevy::prelude::*;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin{
    fn build(&self, app: &mut App){
        app
            .add_state(GameState::Start)
            .add_state(GameState::Play)
            .add_state(GameState::Reset);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState{
    Start,
    Play,
    Reset,
}