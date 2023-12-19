use std::f32::consts::PI;

use crate::prelude::*;
use bevy::prelude::*;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMaterials(None))
            .add_systems(OnExit(GameState::Loading), spawn_tiles)
            .add_systems(OnExit(GameState::InGame), despawn_tiles)
            .add_systems(
                Update,
                highlight_hovered_tile.run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct Tile;

#[derive(Resource)]
struct TileMaterials(Option<TileMaterialHandles>);

struct TileMaterialHandles {
    base: Handle<StandardMaterial>,
    hovered: Handle<StandardMaterial>,
}

fn spawn_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tile_materials: ResMut<TileMaterials>,
) {
    let hexagon = shape::RegularPolygon {
        sides: 6,
        radius: 0.49,
    };
    let hexagon_mesh = meshes.add(hexagon.into());
    let base_material = materials.add(StandardMaterial {
        base_color: Color::CYAN,
        ..default()
    });
    let hovered_material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..default()
    });
    tile_materials.0 = Some(TileMaterialHandles {
        base: base_material.clone(),
        hovered: hovered_material,
    });

    let grid_size = 3;
    for q in -grid_size..=grid_size {
        for r in (-grid_size - q).max(-grid_size)..=(grid_size - q).min(grid_size) {
            let pos = TilePosition::new(q as f32, r as f32);

            commands.spawn((
                PbrBundle {
                    mesh: hexagon_mesh.clone(),
                    material: base_material.clone(),
                    transform: Transform::from_translation((&pos).into()).with_rotation(
                        Quat::from_rotation_x(PI / -2.) * Quat::from_rotation_z(PI / 2.),
                    ),
                    ..default()
                },
                pos.clone(),
                Tile,
            ));
        }
    }
}

fn despawn_tiles(
    mut commands: Commands,
    query: Query<Entity, With<Tile>>,
    mut tile_materials: ResMut<TileMaterials>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    tile_materials.0 = None;
}

fn highlight_hovered_tile(
    mut material_query: Query<(&mut Handle<StandardMaterial>, &TilePosition), With<Tile>>,
    tile_materials: Res<TileMaterials>,
    mouse_position: Res<MousePosition>,
) {
    let hovered_tile = mouse_position.tile.round();
    let Some(tile_material_handles) = tile_materials.0.as_ref() else {
        warn!("Missing tile material handles in resource");
        return;
    };

    for (mut material_handle, pos) in material_query.iter_mut() {
        let is_hovered = *pos == hovered_tile;
        *material_handle = if is_hovered {
            tile_material_handles.hovered.clone()
        } else {
            tile_material_handles.base.clone()
        };
    }
}
