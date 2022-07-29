use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use bevy::{
    core::FixedTimestep,
    ecs::query::WorldQuery,
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision}
};
use rand::Rng;

mod components;
mod enemy;
mod player;

const FLOOR_POSITION: f32 = -350.0;
const CHAR_STARTING_LOCATION: Location = Location {
    x: -600.0,y: FLOOR_POSITION + 15.0, z: 0.0
};

const TILE_MOVE_SIZE: f32 = 5.0;
const MISSILE_TRAVEL: f32 = 20.0;

const LEFT_WALL: f32 = -600.0;
const RIGHT_WALL: f32 = 600.0;
const BOTTOM_WALL: f32 = -400.0;
const TOP_WALL: f32 = 400.0;

const BOARD_WIDTH: f32 = 1200.0;
const BOARD_HEIGHT: f32 = 800.0;

const MISSILE_SIZE: Vec3 = const_vec3!([120.0, 20.0, 0.0]);
const MISSILE_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Copy, Clone)]
pub struct WinSize {
    w: f32,
    h: f32
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Location {
    x: f32,
    y: f32,
    z: f32
}

impl Location {
    fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            self.x,
            self.y,
            self.z
        )
    }
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Character;

use components::Velocity;
use components::VelocityTrait;

#[derive(Component, Debug)]
struct Projectile {
    direction: FacingDirection
}

impl Projectile {
    fn new(direction: FacingDirection) -> Self {
        Self {
            direction
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FacingDirection {
    Left,
    Right,
    Up,
    Down
}

impl Default for FacingDirection {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Component, Default)]
struct Game {
    player: player::Player
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_startup_system(setup_system)

        .add_system_set(
            SystemSet::new()
            .with_system(player_action)
            .with_system(shoot_action)
            .with_system(move_missiles)
        )
        .run();
}

fn random_location() -> Vec3 {
    let mut range = rand::thread_rng();
    let x = range.gen_range(LEFT_WALL..RIGHT_WALL);
    let y = range.gen_range(BOTTOM_WALL..TOP_WALL);

    Vec3::new(x, y, 0.0)
}

// Add the game's entities to our world
fn setup_system(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    windows: Res<Windows>
) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Get window size
    let window = windows.get_primary().unwrap();
    let win_size = WinSize {
        w: window.width(),
        h: window.height()
    };

    // Add window size resource
    commands.insert_resource(win_size.clone());

    // Spawn floor
    let bottom = -win_size.h / 2.0;

    for num in -800..800 {
        let n = num as f32;
        commands
            .spawn()
            .insert(Tile)
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("textures/rpg/tiles/generic-rpg-tile42.png"),
                transform: Transform {
                    translation: Vec3::new(n, bottom , 0.0),
                    ..default()
                },
                ..default()
            });
    };

    game.player.set_direction(FacingDirection::Right);
}

fn player_action(mut _commands: Commands, keyboard_input: Res<Input<KeyCode>>, mut game: ResMut<Game>, mut person_query: Query<&mut Transform, With<player::Player>>) {
    let mut moved = false;
    let mut player_transform = person_query.single_mut();

    let mut new_x = game.player.location().x;
    let mut new_y = game.player.location().y;

    if keyboard_input.pressed(KeyCode::Right) {
        game.player.set_direction(FacingDirection::Right);

        new_x = game.player.location().x + TILE_MOVE_SIZE;
        if new_x < RIGHT_WALL {
            moved = true;
        }
    } else if keyboard_input.pressed(KeyCode::Left) {
        game.player.set_direction(FacingDirection::Left);

        new_x = game.player.location().x - TILE_MOVE_SIZE;
        if new_x > LEFT_WALL {
            moved = true;
        }

    } else if keyboard_input.pressed(KeyCode::Up) {
        game.player.set_direction(FacingDirection::Up);

        new_y = game.player.location().y + TILE_MOVE_SIZE;
        if new_y < TOP_WALL {
            moved = true;
        }
    } else if keyboard_input.pressed(KeyCode::Down) {
        game.player.set_direction(FacingDirection::Down);

        new_y = game.player.location().y - TILE_MOVE_SIZE;
        if new_y > BOTTOM_WALL {
            moved = true;
        }
    }

    if moved {
        player_transform.translation.x = game.player.location().x;
        player_transform.translation.y = game.player.location().y;
        player_transform.translation.z = game.player.location().z;
        game.player.set_location(Some(new_x), Some(new_y));
    }
}

fn shoot_action(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, game: ResMut<Game>) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return
    }

    let missile_location = Location { x: game.player.location().x + 5.0, ..game.player.location() };

    Some(
        commands
        .spawn()
        .insert(Projectile::new(game.player.direction()))
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

fn move_missiles(mut projectile_query: Query<(&mut Transform, &Projectile)>) {
    for (mut transform, projectile) in projectile_query.iter_mut() {
        match projectile.direction {
            FacingDirection::Up => {
                transform.translation.y += MISSILE_TRAVEL;

                // https://github.com/bevyengine/bevy/blob/latest/examples/games/breakout.rs#L370
//                 let collision = collide(
//                     transform.translation,
//                     Vec2::new(10.0, 10.0),
//                     Vec3::new(LEFT_WALL, TOP_WALL + 50.0, 0.0),
//                     Vec2::new(BOARD_WIDTH, 10.0)
//                 );
//
//                 if let Some(collision) = collision {
//                     println!("collision!");
//                     commands.entity(entity).despawn();
//                 }
            },
            FacingDirection::Down => {
                transform.translation.y -= MISSILE_TRAVEL;
            },
            FacingDirection::Left => {
                transform.translation.x -= MISSILE_TRAVEL;
            },
            FacingDirection::Right => {
                transform.translation.x += MISSILE_TRAVEL;
            }
        }
    };
}
