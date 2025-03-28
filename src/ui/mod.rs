pub mod build_button;
pub mod hud;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((build_button::BuildButtonPlugin, hud::HudPlugin));
    }
}
