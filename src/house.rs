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
        app.init_resource::<HouseResource>()
            .add_systems(
                OnExit(GameState::Loading),
                (initialize_house_resource, spawn_house).chain(),
            )
            .add_systems(OnExit(GameState::InGame), cleanup)
            .add_systems(
                Update,
                follow_mouse
                    .run_if(in_state(GameState::InGame))
                    .after(update_mouse_position)
                    .before(set_tile_position),
            );
    }
}

#[derive(Resource, Default)]
struct HouseResource {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn initialize_house_resource(
    mut house_resource: ResMut<HouseResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    house_resource.mesh = meshes.add(shape::Cube { size: 0.4 }.into());
    house_resource.material = materials.add(StandardMaterial {
        base_color: Color::GOLD,
        ..default()
    });
}

#[derive(Component)]
struct House;

impl House {
    const ROTATION_Y: f32 = PI / -6.;
}

fn spawn_house(mut commands: Commands, house_resource: Res<HouseResource>) {
    let position = TilePosition::new(0., 0.);
    let translation = position.to_vec3_with_y(0.2);

    commands.spawn((
        PbrBundle {
            mesh: house_resource.mesh.clone(),
            material: house_resource.material.clone(),
            transform: Transform::from_translation(translation)
                .with_rotation(Quat::from_rotation_y(House::ROTATION_Y)),
            ..default()
        },
        TilePosition::new(0., 0.),
        TranslationSpring::new(translation, 0.15, 0.5),
        ScaleSpring::new(Vec3::ONE, 0.15, 0.5),
        House,
    ));
}

fn cleanup(
    mut commands: Commands,
    house_query: Query<Entity, With<House>>,
    mut house_resource: ResMut<HouseResource>,
) {
    for house_entity in house_query.iter() {
        commands.entity(house_entity).despawn_recursive();
    }

    house_resource.mesh = Default::default();
    house_resource.material = Default::default();
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
