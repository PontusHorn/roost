use bevy::prelude::*;

pub struct DefaultsPlugin;

impl Plugin for DefaultsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Roost".to_string(),
                        resolution: (1024.0, 768.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        );
    }
}
