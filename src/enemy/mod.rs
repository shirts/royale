use crate::{
    BASE_SPEED,
    Game,
    TIME_STEP,
    random_location
};
use crate::components::{
    Velocity,
    VelocityTrait
};

use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_enemy_system)
        .add_system(enemy_movement_system);
    }
}

#[derive(Component)]
struct Enemy;
impl VelocityTrait for Enemy {
    fn velocity() -> Velocity {
        Velocity {
            x: 0.5,
            y: 0.0
        }
    }
}

fn spawn_enemy_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/simplespace/enemy_B.png"),
            transform: Transform {
                translation: random_location(),
                ..default()
            },
            ..default()
        })
    .insert(Enemy)
    .insert(Enemy::velocity());
}

fn enemy_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Enemy>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
