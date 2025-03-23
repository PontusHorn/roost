#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use crate::state::GameState;

pub struct BuildButtonPlugin;

impl Plugin for BuildButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_button);
    }
}

#[derive(Component)]
pub struct BuildButton;

#[derive(Bundle)]
pub struct BuildButtonBundle {
    pub button: ButtonBundle,
    pub colors: BuildButtonColors,
    build_button: BuildButton,
}

impl Default for BuildButtonBundle {
    fn default() -> Self {
        let button_colors = BuildButtonColors::default();
        BuildButtonBundle {
            button: ButtonBundle {
                style: Style {
                    width: Val::Px(80.),
                    height: Val::Px(80.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(8.0)),
                    ..default()
                },
                background_color: BuildButtonColors::BG.into(),
                ..default()
            },
            colors: button_colors,
            build_button: BuildButton,
        }
    }
}

#[derive(Component)]
pub struct BuildButtonLabel;

#[derive(Bundle)]
pub struct BuildButtonLabelBundle {
    text: TextBundle,
    build_button: BuildButtonLabel,
}

impl Default for BuildButtonLabelBundle {
    fn default() -> Self {
        Self {
            text: TextBundle::from_section(
                "Button",
                TextStyle {
                    font_size: 30.,
                    color: BuildButtonColors::FG,
                    ..default()
                },
            ),
            build_button: BuildButtonLabel,
        }
    }
}

impl BuildButtonLabelBundle {
    pub fn new(text: &str, font: Handle<Font>) -> Self {
        let mut bundle = Self::default();
        let default_section = &mut bundle.text.text.sections[0];
        default_section.value = text.to_string();
        default_section.style.font = font.clone();
        bundle
    }
}

fn update_button(
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &BuildButtonColors,
            &Children,
        ),
        With<BuildButton>,
    >,
    mut label_query: Query<&mut Text, With<BuildButtonLabel>>,
    game_state: Res<State<GameState>>,
) {
    for (interaction, mut background_color, colors, children) in &mut button_query {
        let (bg, fg) = match (*interaction, game_state.get()) {
            (_, GameState::Building) => colors.on,
            (Interaction::Hovered, _) => colors.hovered,
            (Interaction::Pressed, _) => colors.pressed,
            (Interaction::None, _) => colors.off,
        };

        background_color.0 = bg;
        for child in children.iter() {
            if let Ok(mut style) = label_query.get_mut(*child) {
                for section in &mut style.sections {
                    section.style.color = fg;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct BuildButtonColors {
    pub off: (Color, Color),
    pub on: (Color, Color),
    pub hovered: (Color, Color),
    pub pressed: (Color, Color),
}

impl BuildButtonColors {
    pub const BG: Color = Color::rgb(0.3, 0.19, 0.07);
    pub const FG: Color = Color::rgb(0.9, 0.9, 0.9);
}

impl Default for BuildButtonColors {
    fn default() -> Self {
        let base_lightness = Self::BG.l();
        BuildButtonColors {
            off: (Self::BG, Self::FG),
            on: (Color::BLUE, Color::WHITE),
            hovered: (Self::BG.with_l(base_lightness * 1.5), Color::WHITE),
            pressed: (Self::BG.with_l(base_lightness / 2.), Color::WHITE),
        }
    }
}
