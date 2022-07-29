use bevy::prelude::*;

#[derive(Component)]
pub struct FromEnemy;

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

pub trait VelocityTrait {
    fn velocity() -> Velocity {
        Velocity {
            x: 1.0,
            y: 0.0
        }
    }
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool
}

impl Movable {
    pub fn new(auto_despawn: bool) -> Self {
        Self {
            auto_despawn
        }
    }
}

#[derive(Component, Debug)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

