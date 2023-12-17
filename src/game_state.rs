use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::InGame),
        );
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    InGame,
}
