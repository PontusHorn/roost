use crate::{camera::MainCamera, prelude::*};
use bevy::prelude::*;

pub struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>().add_systems(
            Update,
            update_mouse_position.run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Resource, Debug, PartialEq)]
pub struct MousePosition {
    pub viewport: Vec2,
    pub world: Vec3,
    pub tile: TilePosition,
}

impl Default for MousePosition {
    fn default() -> Self {
        Self {
            viewport: Vec2::ZERO,
            world: Vec3::ZERO,
            tile: TilePosition::ZERO,
        }
    }
}

pub fn update_mouse_position(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_query: Query<&Window>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    let Some(viewport_position) = window.cursor_position() else {
        return;
    };

    let Some(world_position) = camera
        .viewport_to_world(camera_transform, viewport_position)
        .and_then(|world_ray| {
            world_ray
                .intersect_plane(Vec3::ZERO, Vec3::Y)
                .map(|distance| world_ray.get_point(distance))
        })
    else {
        return;
    };

    let tile_position = TilePosition::from(world_position);
    let current_mouse_position = MousePosition {
        viewport: viewport_position,
        world: world_position,
        tile: tile_position,
    };

    if current_mouse_position != *mouse_position {
        *mouse_position = current_mouse_position;
    }
}
