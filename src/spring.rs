use crate::prelude::*;
use bevy::prelude::*;

pub struct SpringPlugin;

impl Plugin for SpringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_translation_springs, apply_scale_springs).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component, Debug)]
pub struct TranslationSpring {
    pub target: Vec3,
    pub velocity: Vec3,
    pub stiffness: f32,
    pub damping: f32,
}

impl TranslationSpring {
    pub fn new(target: Vec3, stiffness: f32, damping: f32) -> Self {
        Self {
            target,
            velocity: Vec3::ZERO,
            stiffness,
            damping,
        }
    }
}

impl Default for TranslationSpring {
    fn default() -> Self {
        Self::new(Vec3::ZERO, 0.2, 0.5)
    }
}

pub fn apply_translation_springs(
    mut query: Query<(&mut Transform, &mut TranslationSpring)>,
    time: Res<Time>,
) {
    let time_factor = time.delta_seconds() * 60.0;
    for (mut transform, mut spring) in query.iter_mut() {
        let force = (spring.target - transform.translation) * spring.stiffness;
        let damping = spring.velocity * spring.damping;
        let acceleration = (force - damping) / 1.0;
        spring.velocity += acceleration * time_factor;
        transform.translation += spring.velocity * time_factor;
        if (spring.target - transform.translation).length() < 0.001
            && spring.velocity.length() < 0.001
        {
            transform.translation = spring.target;
            spring.velocity = Vec3::ZERO;
        }
    }
}

#[derive(Component, Debug)]
pub struct ScaleSpring {
    pub target: Vec3,
    pub velocity: Vec3,
    pub stiffness: f32,
    pub damping: f32,
}

impl ScaleSpring {
    pub fn new(target: Vec3, stiffness: f32, damping: f32) -> Self {
        Self {
            target,
            velocity: Vec3::ZERO,
            stiffness,
            damping,
        }
    }
}

impl Default for ScaleSpring {
    fn default() -> Self {
        Self::new(Vec3::ZERO, 0.2, 0.5)
    }
}

pub fn apply_scale_springs(mut query: Query<(&mut Transform, &mut ScaleSpring)>) {
    for (mut transform, mut spring) in query.iter_mut() {
        let force = (spring.target - transform.scale) * spring.stiffness;
        let damping = spring.velocity * spring.damping;
        let acceleration = (force - damping) / 1.0;
        spring.velocity += acceleration;
        transform.scale += spring.velocity;
        if (spring.target - transform.scale).length() < 0.001 && spring.velocity.length() < 0.001 {
            transform.scale = spring.target;
            spring.velocity = Vec3::ZERO;
        }
    }
}
