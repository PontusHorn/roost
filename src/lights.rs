use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

pub struct LightsPlugin;

impl Plugin for LightsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb(1., 1., 1.),
        brightness: 0.1,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-0.7, 2.0, 0.7))
            .looking_at(Vec3::ZERO, Vec3::Y),
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            range: 15.,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 2.0, 8.0),
        ..default()
    });
}
