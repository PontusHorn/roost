use crate::{prelude::*, spring::TranslationSpring};
use bevy::prelude::*;

pub struct TilePositionPlugin;

impl Plugin for TilePositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            set_tile_position.run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component, Debug, PartialEq, Clone)]
pub struct TilePosition {
    pub q: f32,
    pub r: f32,
}

const SQRT_3: f32 = 1.7320508;

impl TilePosition {
    pub const CIRCUMRADIUS: f32 = 0.5;
    pub const INRADIUS: f32 = SQRT_3 * Self::CIRCUMRADIUS / 2.0;

    /// A zero tile vector.
    pub const ZERO: Self = Self::new(0., 0.);

    /// A tile vector pointing one tile "north".
    pub const N: Self = Self::new(0., -1.);

    /// A tile vector pointing one tile "north-east".
    pub const NE: Self = Self::new(1., -1.);

    /// A tile vector pointing one tile "south-east".
    pub const SE: Self = Self::new(1., 0.);

    /// A tile vector pointing one tile "south".
    pub const S: Self = Self::new(0., 1.);

    /// A tile vector pointing one tile "south-west".
    pub const SW: Self = Self::new(-1., 1.);

    /// A tile vector pointing one tile "north-west".
    pub const NW: Self = Self::new(-1., 0.);

    /// The "north" tile vector as a `Vec2` in world space.
    pub const N_VEC2: Vec2 = Vec2::new(0., Self::INRADIUS * 2.0);

    /// The "north-east" tile vector as a `Vec2` in world space.
    pub const NE_VEC2: Vec2 = Vec2::new(Self::CIRCUMRADIUS, Self::INRADIUS);

    /// The "south-east" tile vector as a `Vec2` in world space.
    pub const SE_VEC2: Vec2 = Vec2::new(Self::CIRCUMRADIUS, -Self::INRADIUS);

    /// The "south" tile vector as a `Vec2` in world space.
    pub const S_VEC2: Vec2 = Vec2::new(0., -Self::INRADIUS * 2.0);

    /// The "south-west" tile vector as a `Vec2` in world space.
    pub const SW_VEC2: Vec2 = Vec2::new(-Self::CIRCUMRADIUS, -Self::INRADIUS);

    /// The "north-west" tile vector as a `Vec2` in world space.
    pub const NW_VEC2: Vec2 = Vec2::new(-Self::CIRCUMRADIUS, Self::INRADIUS);

    /// The Q basis vector as a `Vec2` in world space.
    pub const Q_VEC2: Vec2 = Vec2::new(3. / 2., SQRT_3 / 2.);

    /// The R basis vector as a `Vec2` in world space.
    pub const R_VEC2: Vec2 = Vec2::new(0., SQRT_3);

    pub const fn new(q: f32, r: f32) -> Self {
        Self { q, r }
    }

    pub fn s(&self) -> f32 {
        -self.q - self.r
    }

    pub fn round(&self) -> Self {
        let mut q = self.q.round();
        let mut r = self.r.round();
        let s = self.s().round();
        let q_diff = (q - self.q).abs();
        let r_diff = (r - self.r).abs();
        let s_diff = (s - self.s()).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        }

        Self { q, r }
    }

    pub fn corners(&self) -> impl Iterator<Item = Vec2> + '_ {
        let center = Vec2::from(self.round());
        (0..6).map(move |i| {
            let angle = std::f32::consts::PI / 3.0 * i as f32;
            center + Vec2::new(angle.cos(), angle.sin()) * Self::CIRCUMRADIUS
        })
    }

    pub fn to_vec3_with_y(&self, y: f32) -> Vec3 {
        let pos = Vec2::from(self);
        Vec3::new(pos.x, y, -pos.y)
    }
}

impl<'a> From<&'a TilePosition> for Vec2 {
    fn from(TilePosition { q, r }: &'a TilePosition) -> Self {
        (TilePosition::Q_VEC2 * *q + TilePosition::R_VEC2 * *r) * TilePosition::CIRCUMRADIUS
    }
}

impl From<TilePosition> for Vec2 {
    /// Translate a tile position in the hexagonal grid into a world position
    fn from(pos: TilePosition) -> Vec2 {
        Vec2::from(&pos)
    }
}

impl<'a> From<&'a TilePosition> for Vec3 {
    fn from(pos: &'a TilePosition) -> Vec3 {
        pos.to_vec3_with_y(0.)
    }
}

impl From<TilePosition> for Vec3 {
    fn from(pos: TilePosition) -> Vec3 {
        Vec3::from(&pos)
    }
}

impl<'a> From<&'a Vec2> for TilePosition {
    /// Translate a world position into a tile position in the hexagonal grid
    fn from(Vec2 { x, y }: &'a Vec2) -> Self {
        let q = (2. / 3. * x) / Self::CIRCUMRADIUS;
        let r = (-1. / 3. * x + SQRT_3 / 3. * y) / Self::CIRCUMRADIUS;
        TilePosition::new(q, r)
    }
}

impl From<Vec2> for TilePosition {
    fn from(value: Vec2) -> Self {
        TilePosition::from(&value)
    }
}

impl<'a> From<&'a Vec3> for TilePosition {
    fn from(Vec3 { x, z, .. }: &'a Vec3) -> Self {
        Vec2::new(*x, -*z).into()
    }
}

impl From<Vec3> for TilePosition {
    fn from(Vec3 { x, z, .. }: Vec3) -> Self {
        Vec2::new(x, -z).into()
    }
}

/// Updates the position of any entity with a tile position and a transform
/// component to the center of the tile position's world space equivalent. Only
/// runs when the tile position changes.
pub fn set_tile_position(
    mut query: Query<
        (
            &mut Transform,
            &TilePosition,
            Option<&mut TranslationSpring>,
        ),
        Changed<TilePosition>,
    >,
) {
    for (mut transform, tile_position, translation_spring) in query.iter_mut() {
        let Vec2 { x, y } = Vec2::from(tile_position);
        match translation_spring {
            None => {
                transform.translation.x = x;
                transform.translation.z = -y;
            }
            Some(mut translation_spring) => {
                translation_spring.target = Vec3::new(x, transform.translation.y, -y);
            }
        }
    }
}
