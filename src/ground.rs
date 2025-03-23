use crate::prelude::*;
use bevy::prelude::*;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), spawn_ground)
            .add_systems(OnExit(AppState::InGame), despawn_ground);
    }
}

#[derive(Component)]
struct Ground;

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_mesh = meshes.add(shape::Plane::from_size(100.).into());
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.3, 0.19, 0.07),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: plane_mesh.clone(),
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(0., -0.001, 0.)),
            ..default()
        },
        Ground,
    ));
}

fn despawn_ground(mut commands: Commands, ground_query: Query<Entity, With<Ground>>) {
    for ground_entity in ground_query.iter() {
        commands.entity(ground_entity).despawn_recursive();
    }
}
