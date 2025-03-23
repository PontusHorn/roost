mod camera;
mod defaults;
mod ground;
mod lights;
mod mouse_position;
mod prelude;
mod spring;
mod state;
mod structure;
mod tile_position;
mod tiles;
mod ui;

use bevy::prelude::*;
use ui::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            camera::CameraPlugin,
            lights::LightsPlugin,
            defaults::DefaultsPlugin,
            UiPlugin,
            state::StatePlugin,
            spring::SpringPlugin,
            // WorldInspectorPlugin::new(),
            mouse_position::MousePositionPlugin,
            ground::GroundPlugin,
            structure::StructurePlugin,
            tiles::TilesPlugin,
            tile_position::TilePositionPlugin,
        ))
        .run();
}
