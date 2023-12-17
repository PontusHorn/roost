use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::MIDNIGHT_BLUE),
            },
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                ..default()
            },
            ..default()
        },
        MainCamera,
        Name::new("Camera"),
    ));
}
