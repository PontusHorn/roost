use crate::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct TileGizmosPlugin;

impl Plugin for TileGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), spawn_tile_gizmos)
            .add_systems(
                Update,
                highlight_hovered_tile.run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
struct Tile;

pub const TRANSPARENT: Color = Color::rgba(0., 0., 0., 0.);

fn spawn_tile_gizmos(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        color: Color::BLACK,
        font: asset_server.load("fonts/itim.ttf"),
        font_size: 20.0,
    };

    let grid_size = 3;
    for q in -grid_size..=grid_size {
        for r in (-grid_size - q).max(-grid_size)..=(grid_size - q).min(grid_size) {
            let pos = TilePosition::new(q as f32, r as f32);
            let shape = shapes::RegularPolygon {
                sides: 6,
                feature: shapes::RegularPolygonFeature::Radius(TilePosition::CIRCUMRADIUS - 2.),
                ..default()
            };

            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    spatial: SpatialBundle {
                        transform: Transform::from_translation(Vec2::from(&pos).extend(-1.)),
                        ..default()
                    },
                    ..default()
                },
                Fill::color(Color::CYAN),
                Stroke {
                    color: TRANSPARENT,
                    options: StrokeOptions::default()
                        .with_line_width(6.)
                        .with_line_join(LineJoin::Round)
                        .with_miter_limit(1.),
                },
                pos.clone(),
                Tile,
            ));

            let text = format!("{}, {}", pos.q, pos.r);
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(text, text_style.clone()),
                    transform: Transform::from_translation((&pos).into()),
                    ..default()
                },
                pos,
            ));
        }
    }
}

fn highlight_hovered_tile(
    mut strokes_query: Query<(&mut Stroke, &TilePosition), With<Tile>>,
    mouse_position: Res<MousePosition>,
) {
    let hovered_tile = mouse_position.tile.round();

    for (mut stroke, pos) in strokes_query.iter_mut() {
        let is_hovered = *pos == hovered_tile;
        stroke.color = if is_hovered {
            Color::BLACK
        } else {
            TRANSPARENT
        };
    }
}
