pub mod house;
mod planned_structure;

use bevy::prelude::*;

use crate::state::AppState;

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsByStructureType>()
            .add_plugins(house::HousePlugin)
            .add_plugins(planned_structure::PlannedStructurePlugin)
            .add_systems(OnExit(AppState::InGame), cleanup);
    }
}

#[derive(Clone)]
pub enum StructureType {
    House,
}

pub trait Structure {
    fn assets(&self, world: &mut World) -> Option<StructureAssets>;
    fn transform(&self) -> Transform;
}

#[derive(Default, Clone)]
pub struct StructureAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Resource, Default, Clone)]
pub struct AssetsByStructureType {
    pub house: StructureAssets,
}

fn cleanup(mut assets: ResMut<AssetsByStructureType>) {
    *assets = Default::default();
}
