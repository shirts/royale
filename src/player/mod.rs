use crate::{
    BASE_SPEED,
    PLAYER_SPRITE_SCALE,
    PLAYER_STARTING_LOCATION,
    MISSILE_COLOR,
    MISSILE_SIZE,
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

use crate::components::Movable;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
        .add_system(player_keyboard_event_system)
        .add_system(player_shoot_system);
    }
}

#[derive(Component, Copy, Clone, Default)]
pub struct Player {
    entity: Option<Entity>,
    direction: FacingDirection,
    fire_delay: i32
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

    pub fn fire_delay(&self) -> i32 {
        self.fire_delay
    }

    pub fn reduce_fire_delay(&mut self) {
        let new_delay = self.fire_delay() - 1;
        if (new_delay < 0) { return };

        self.set_fire_delay(new_delay);
    }

    fn set_fire_delay(&mut self, fire_delay: i32) {

        self.fire_delay = fire_delay;
    }
}

fn spawn_player_system(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>
) {

    game.player.set_direction(FacingDirection::Right);

    game.player.set_fire_delay(0);

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

fn player_shoot_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut query: Query<&mut Transform, With<Player>>,
    ) {

    // Reduce delay each tick
    game.player.reduce_fire_delay();

    // Return if player can't fire
    if game.player.fire_delay() > 0 {
        return
    }

    if !keyboard_input.pressed(KeyCode::Space) {
        return
    }

    // Set fire delay
    game.player.set_fire_delay(10);

    if let Ok(transform) = query.get_single_mut() {
        let (x, y) = (transform.translation.x, transform.translation.y);
        let x_offset = PLAYER_SPRITE_SIZE.0 / 2.0 * PLAYER_SPRITE_SCALE - 5.0;
        let y_offset = 15.0;

        let missile_location = Location { x: x + x_offset, y: y + y_offset, z: 0.0 };

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

