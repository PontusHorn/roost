mod camera;
mod defaults;
mod game_state;
mod mouse_position;
mod prelude;
mod tile_gizmos;
mod tile_position;

use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            camera::CameraPlugin,
            defaults::DefaultsPlugin,
            game_state::GameStatePlugin,
            // WorldInspectorPlugin::new(),
            mouse_position::MousePositionPlugin,
            ShapePlugin,
            tile_gizmos::TileGizmosPlugin,
            tile_position::TilePositionPlugin,
        ))
        .run();
}
