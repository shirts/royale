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
