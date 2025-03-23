use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_state::<GameState>()
            .add_systems(OnEnter(AppState::InGame), update_state(GameState::Playing))
            .add_systems(OnExit(AppState::InGame), update_state(GameState::None))
            .add_loading_state(
                LoadingState::new(AppState::Loading).continue_to_state(AppState::InGame),
            );
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    None,
    Playing,
    Building,
}

pub fn update_state<T: States>(state: T) -> impl Fn(ResMut<NextState<T>>) {
    move |mut next_state: ResMut<NextState<T>>| {
        next_state.set(state.clone());
    }
}
