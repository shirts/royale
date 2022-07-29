use crate::{
    BASE_SPEED,
    PLAYER_STARTING_LOCATION,
    MISSILE_COLOR,
    MISSILE_SIZE,
    MISSILE_SPRITE,
    PLAYER_SPRITE_SIZE,
    TIME_STEP,
    Game,
    Projectile,
    random_location
};
use crate::{
    FacingDirection,
    Location,
    Velocity,
    VelocityTrait
};

use crate::components::{
    Movable
};

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
        .add_system(player_keyboard_event_system)
        .add_system(player_movement_system)
        .add_system(player_shoot_system);
    }
}

#[derive(Component, Copy, Clone, Default)]
pub struct Player {
    entity: Option<Entity>,
    direction: FacingDirection
}

impl VelocityTrait for Player {
    fn velocity() -> Velocity {
        Velocity {
            x: 50.0,
            y: 3.0
        }
    }
}

impl Player {
    pub fn direction(&self) -> FacingDirection {
        self.direction
    }

    pub fn set_direction(&mut self, direction: FacingDirection) {
        self.direction = direction;
    }
}

fn spawn_player_system(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>
) {
    game.player.entity = Some(
        commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/rpg/chars/sensei/sensei.png"),
            transform: Transform {
                translation: random_location(),
                ..default()
            },
            ..default()
        })
        .insert(game.player)
        .insert(Player::velocity())
        .insert(Movable::new(false))
        .id()
    );
}

fn player_keyboard_event_system(mut game: ResMut<Game>, kb: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x =
            if kb.pressed(KeyCode::Left) {
                game.player.set_direction(FacingDirection::Left);
                -1.0
            } else if kb.pressed(KeyCode::Right) {
                game.player.set_direction(FacingDirection::Right);
                1.0
            } else {
                0.0
            };

        velocity.y =
            if kb.pressed(KeyCode::Up) {
                game.player.set_direction(FacingDirection::Up);
                1.0
            } else if kb.pressed(KeyCode::Down) {
                game.player.set_direction(FacingDirection::Down);
                -1.0
            } else {
                0.0
            };
    }
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}

fn player_shoot_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game: ResMut<Game>,
    mut query: Query<&mut Transform, With<Player>>,
    ) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return
    }

    if let Ok(transform) = query.get_single_mut() {
        let (x, y) = (transform.translation.x, transform.translation.y);

        let missile_location = Location { x: x + PLAYER_SPRITE_SIZE, y, z: 0.0 };

        Some(
            commands
            .spawn()
            .insert(Projectile::new(game.player.direction()))
            .insert(Projectile::velocity())
            .insert(Movable::new(true))
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(missile_location.x, missile_location.y, missile_location.z),
                    scale: MISSILE_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color: MISSILE_COLOR,
                    ..default()
                },
                ..default()
            })
            .id()
        );
    }
}

