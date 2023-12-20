use std::f32::consts::PI;

use crate::{
    mouse_position::update_mouse_position,
    prelude::*,
    spring::{ScaleSpring, TranslationSpring},
    tile_position::set_tile_position,
    tiles::Tile,
};
use bevy::prelude::*;

pub struct HousePlugin;

impl Plugin for HousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), spawn_house)
            .add_systems(OnExit(GameState::InGame), despawn_house)
            .add_systems(
                Update,
                follow_mouse
                    .run_if(in_state(GameState::InGame))
                    .after(update_mouse_position)
                    .before(set_tile_position),
            );
    }
}

#[derive(Component)]
struct House;

fn spawn_house(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = meshes.add(shape::Cube { size: 0.4 }.into());
    let material = materials.add(StandardMaterial {
        base_color: Color::GOLD,
        ..default()
    });
    let position = TilePosition::new(0., 0.);
    let translation = position.to_vec3_with_y(0.2);

    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: material.clone(),
            transform: Transform::from_translation(translation)
                .with_rotation(Quat::from_rotation_y(PI / -6.)),
            ..default()
        },
        TilePosition::new(0., 0.),
        TranslationSpring::new(translation, 0.15, 0.5),
        ScaleSpring::new(Vec3::ONE, 0.15, 0.5),
        House,
    ));
}

fn despawn_house(mut commands: Commands, house_query: Query<Entity, With<House>>) {
    for house_entity in house_query.iter() {
        commands.entity(house_entity).despawn_recursive();
    }
}

fn follow_mouse(
    mut house_query: Query<(&mut TilePosition, &mut ScaleSpring), With<House>>,
    tile_query: Query<&TilePosition, (With<Tile>, Without<House>)>,
    mouse_position: Res<MousePosition>,
) {
    let hovered_tile = mouse_position.tile.round();
    let is_on_tile = tile_query
        .iter()
        .any(|tile_position| *tile_position == hovered_tile);

    for (mut tile_position, mut scale_spring) in house_query.iter_mut() {
        *tile_position = hovered_tile.clone();
        scale_spring.target = if is_on_tile { Vec3::ONE } else { Vec3::ZERO };
    }
}
