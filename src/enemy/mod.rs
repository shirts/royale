use crate::{
    BASE_SPEED,
    TIME_STEP,
    random_location
};
use crate::components::{
    Velocity,
    VelocityTrait
};

use bevy::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_enemy_system)
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

fn should_spawn_enemy() -> bool {
    let mut rng = thread_rng();
    let choices = [true, false, false, false, false];
    *choices.choose(&mut rng).unwrap()
}

fn spawn_enemy_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    if !should_spawn_enemy() {
        println!("not spawning enemy");
        return
    }

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
