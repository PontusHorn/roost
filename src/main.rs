mod camera;
mod defaults;
mod game_state;
mod ground;
mod house;
mod lights;
mod mouse_position;
mod prelude;
mod spring;
mod tile_position;
mod tiles;

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
            spring::SpringPlugin,
            // WorldInspectorPlugin::new(),
            mouse_position::MousePositionPlugin,
            ground::GroundPlugin,
            tiles::TilesPlugin,
            house::HousePlugin,
            tile_position::TilePositionPlugin,
        ))
        .run();
}
