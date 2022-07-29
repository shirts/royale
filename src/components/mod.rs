use bevy::prelude::Component;

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
