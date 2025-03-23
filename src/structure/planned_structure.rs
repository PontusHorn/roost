use crate::{
    mouse_position::update_mouse_position,
    prelude::*,
    spring::{ScaleSpring, TranslationSpring},
    state::GameState,
    structure::house::HouseBundle,
    tile_position::set_tile_position,
    tiles::Tile,
    ui::hud::BuildOnPress,
};
use bevy::prelude::*;

use super::{AssetsByStructureType, StructureType};

pub struct PlannedStructurePlugin;

impl Plugin for PlannedStructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::InGame), cleanup)
            .add_systems(
                Update,
                (
                    spawn_planned_structure.run_if(in_state(GameState::Playing)),
                    (
                        build_structure,
                        follow_mouse
                            .after(update_mouse_position)
                            .before(set_tile_position),
                    )
                        .run_if(in_state(GameState::Building)),
                ),
            );
    }
}

#[derive(Component)]
struct PlannedStructure;

#[derive(Bundle)]
pub struct PlannedStructureBundle {
    tile_position: TilePosition,
    translation_spring: TranslationSpring,
    scale_spring: ScaleSpring,
    planned_structure: PlannedStructure,
}

impl PlannedStructureBundle {
    fn from_tile_position(tile_position: TilePosition) -> Self {
        let translation = tile_position.to_vec3_with_y(0.1);
        Self {
            tile_position,
            translation_spring: TranslationSpring::new(translation, 0.15, 0.5),
            scale_spring: ScaleSpring::new(Vec3::ONE, 0.15, 0.5),
            planned_structure: PlannedStructure,
        }
    }
}

fn spawn_planned_structure(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &BuildOnPress)>,
    assets: Res<AssetsByStructureType>,
    mut next_state: ResMut<NextState<GameState>>,
    mouse_position: Res<MousePosition>,
) {
    for (interaction, build_button) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let mut structure = match build_button.0 {
            StructureType::House => HouseBundle::from_assets(&assets.house),
        };

        structure.pbr.transform.translation = mouse_position.world;

        let planned_structure =
            PlannedStructureBundle::from_tile_position(mouse_position.tile.round());

        commands.spawn((structure, planned_structure));
        next_state.set(GameState::Building);
    }
}

fn cleanup(mut commands: Commands, structure_query: Query<Entity, With<PlannedStructure>>) {
    for structure_entity in structure_query.iter() {
        commands.entity(structure_entity).despawn_recursive();
    }
}

fn follow_mouse(
    mut house_query: Query<(&mut TilePosition, &mut ScaleSpring), With<PlannedStructure>>,
    tile_query: Query<&TilePosition, (With<Tile>, Without<PlannedStructure>)>,
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

fn build_structure(
    mut commands: Commands,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut structure_query: Query<
        (Entity, &mut TilePosition, &mut TranslationSpring),
        With<PlannedStructure>,
    >,
    tile_query: Query<&TilePosition, (With<Tile>, Without<PlannedStructure>)>,
    mouse_position: Res<MousePosition>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if *state.get() != GameState::Building || !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let hovered_tile = mouse_position.tile.round();
    let is_on_tile = tile_query
        .iter()
        .any(|tile_position| *tile_position == hovered_tile);
    if !is_on_tile {
        return;
    }

    for (structure_entity, mut tile_position, mut translation_spring) in structure_query.iter_mut()
    {
        *tile_position = hovered_tile.clone();
        translation_spring.target.y = 0.;
        commands
            .entity(structure_entity)
            .remove::<PlannedStructure>();
    }

    next_state.set(GameState::Playing);
}
