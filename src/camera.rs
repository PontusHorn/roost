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
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::MIDNIGHT_BLUE),
                ..default()
            },
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scale: 0.01,
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(0., 12., 12.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
        MainCamera,
        Name::new("Camera"),
    ));
}
