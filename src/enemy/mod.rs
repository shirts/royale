use crate::{
    BASE_SPEED,
    SPRITE_SCALE,
    TIME_STEP,
    Difficulty,
    Game,
    WinSize,
    random_location
};
use crate::components::{
    Movable,
    SpriteSize,
    Velocity,
    VelocityTrait
};

use bevy::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

const SPRITE_SIZE: (f32, f32) = (crate::SPRITE_SIZE.0 / 1.5, crate::SPRITE_SIZE.0 / 1.5);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(spawn_enemy_system)
        .add_system(enemy_movement_system);
    }
}

#[derive(Component)]
pub struct Enemy;
impl VelocityTrait for Enemy {
    fn velocity() -> Velocity {
        Velocity {
            x: 0.5,
            y: 0.0
        }
    }
}

fn should_spawn_enemy(difficulty: Difficulty) -> bool {
    let mut rng = thread_rng();

    let choices = match difficulty {
        Difficulty::Novice => {
            vec![true, false, false, false, false]
        },
        Difficulty::Hard => {
            vec![true, false]
        },
        Difficulty::Expert => {
            vec![true]
        }
    };

    *choices.choose(&mut rng).unwrap()
}

fn spawn_enemy_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<Game>,
    win_size: Res<WinSize>
) {
    if !should_spawn_enemy(game.difficulty) {
        return
    }

    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/simplespace/enemy_B.png"),
            transform: Transform {
                scale: Vec3::new(SPRITE_SCALE / 4.0, SPRITE_SCALE / 4.0, 1.0),
                translation: random_location(*win_size),
                ..default()
            },
            ..default()
        })
    .insert(Enemy)
    .insert(Enemy::velocity())
    .insert(Movable::new(true))
    .insert(SpriteSize::from(SPRITE_SIZE))
    .id();
}

fn enemy_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Enemy>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
