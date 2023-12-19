mod camera;
mod defaults;
mod game_state;
mod lights;
mod mouse_position;
mod prelude;
mod tile_gizmos;
mod tile_position;

use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            camera::CameraPlugin,
            lights::LightsPlugin,
            defaults::DefaultsPlugin,
            game_state::GameStatePlugin,
            // WorldInspectorPlugin::new(),
            mouse_position::MousePositionPlugin,
            tile_gizmos::TileGizmosPlugin,
            tile_position::TilePositionPlugin,
        ))
        .run();
}
