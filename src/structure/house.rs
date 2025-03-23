use std::f32::consts::PI;

use crate::prelude::*;
use bevy::prelude::*;

use super::{AssetsByStructureType, StructureAssets};

pub struct HousePlugin;

impl Plugin for HousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), initialize_house_assets);
    }
}

fn initialize_house_assets(
    mut assets: ResMut<AssetsByStructureType>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let shape = shape::Box::from_corners(Vec3::new(-0.2, 0., -0.2), Vec3::new(0.2, 0.4, 0.2));
    assets.house.mesh = meshes.add(shape.into());
    assets.house.material = materials.add(StandardMaterial {
        base_color: Color::GOLD,
        ..default()
    });
}

#[derive(Component)]
struct House;

#[derive(Bundle)]
pub struct HouseBundle {
    pub pbr: PbrBundle,
    house: House,
}

impl HouseBundle {
    pub fn from_assets(assets: &StructureAssets) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::from_rotation(Quat::from_rotation_y(PI / -6.))
                    .with_scale(Vec3::ZERO),
                ..default()
            },
            house: House,
        }
    }
}

// fn spawn_house(mut commands: Commands, house_assets: Res<HouseAssets>) {
//     let position = TilePosition::new(0., 0.);
//     let translation = position.to_vec3_with_y(0.2);

//     commands.spawn((
//         PbrBundle {
//             mesh: house_assets.mesh.clone(),
//             material: house_assets.material.clone(),
//             transform: Transform::from_translation(translation)
//                 .with_rotation(Quat::from_rotation_y(House::ROTATION_Y)),
//             ..default()
//         },
//         TilePosition::new(0., 0.),
//         TranslationSpring::new(translation, 0.15, 0.5),
//         ScaleSpring::new(Vec3::ONE, 0.15, 0.5),
//         House,
//     ));
// }
