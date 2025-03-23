use bevy::prelude::*;

use crate::{state::AppState, structure::StructureType};

use super::build_button::{BuildButtonBundle, BuildButtonLabelBundle};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), spawn_hud)
            .add_systems(OnExit(AppState::InGame), despawn_hud);
    }
}

#[derive(Component)]
struct Hud;

#[derive(Component, Clone)]
pub struct BuildOnPress(pub StructureType);

fn spawn_hud(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/AxeHandel.ttf");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Hud,
        ))
        .with_children(|commands| {
            commands
                .spawn((
                    BuildButtonBundle::default(),
                    BuildOnPress(StructureType::House),
                ))
                .with_children(|commands| {
                    commands.spawn(BuildButtonLabelBundle::new("House", font));
                });
        });
}

fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<Hud>>) {
    for hud_entity in hud_query.iter() {
        commands.entity(hud_entity).despawn_recursive();
    }
}
