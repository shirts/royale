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

use components::{
    FromPlayer,
    Movable,
    SpriteSize
};

use enemy::Enemy;

const FLOOR_POSITION: f32 = -350.0;
const PLAYER_STARTING_LOCATION: Location = Location {
    x: -600.0,y: FLOOR_POSITION + 15.0, z: 0.0
};
const SPRITE_SIZE: (f32, f32) = (5.0, 5.0);
const SPRITE_SCALE: f32 = 2.2;

const TILE_MOVE_SIZE: f32 = 5.0;
const MISSILE_TRAVEL: f32 = 20.0;

const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;
const BASE_PROJECTILE_SPEED: f32 = 250.0;

const LEFT_WALL: f32 = -600.0;
const RIGHT_WALL: f32 = 600.0;
const BOTTOM_WALL: f32 = -400.0;
const TOP_WALL: f32 = 400.0;

const BOARD_WIDTH: f32 = 1200.0;
const BOARD_HEIGHT: f32 = 800.0;

const MISSILE_SIZE: Vec3 = const_vec3!([10.0, 15.0, 10.0]);
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

impl VelocityTrait for Projectile {
    fn velocity() -> Velocity {
        Velocity {
            x: 0.0,
            y: 0.0
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug)]
pub enum Difficulty {
    Novice,
    Hard,
    Expert
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::Novice
    }
}

#[derive(Component, Default)]
struct Game {
    player: player::Player,
    difficulty: Difficulty
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(WindowDescriptor {
            width: 1400.0,
            height: 1000.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_system(projectile_velocity_system)
        .add_system(player_projectile_enemy_system)
        .run();
}

fn random_location(win_size: WinSize) -> Vec3 {
    let w_span = win_size.w / 2.0 - 100.0;
    let h_span = win_size.h / 2.0 - 100.0;

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-w_span..w_span);
    let y = rng.gen_range(-h_span..h_span);

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
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>

) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            const MARGIN: f32 = 200.0;
            if translation.y > win_size.h / 2.0 + MARGIN ||
                translation.y < -win_size.h / 2.0 - MARGIN ||
                translation.x > win_size.w / 2.0 + MARGIN ||
                translation.x < -win_size.w / 2.0 - MARGIN
            {
                commands.entity(entity).despawn();
            }

        }
    }
}

fn player_projectile_enemy_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &SpriteSize), (With<Projectile>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {

    // iterate through projectiles
    for (proj_entity, proj_transform, proj_size) in projectile_query.iter() {
        let proj_scale = Vec2::new(proj_transform.scale[0], proj_transform.scale[1]);

        // iterate through enemies
        for (enemy_entity, enemy_transform, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::new(enemy_transform.scale[0], enemy_transform.scale[1]);

            // determine collision
             let collision = collide(
                 proj_transform.translation,
                 proj_size.0 * proj_scale,
                 enemy_transform.translation,
                 enemy_size.0 * enemy_scale
             );

             if let Some(_) = collision {
                 println!("collision. despawning projectile and enemy");
                 commands.entity(enemy_entity).despawn();
                 commands.entity(proj_entity).despawn();
             };


        }
    }
}

fn projectile_velocity_system(mut game: ResMut<Game>, kb: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &Projectile)>) {
    for (mut velocity, projectile) in query.iter_mut() {
        velocity.x =
            if projectile.direction == FacingDirection::Left {
                -1.0 * TIME_STEP * BASE_PROJECTILE_SPEED
            } else if projectile.direction == FacingDirection::Right {
                1.0 * TIME_STEP * BASE_PROJECTILE_SPEED
            } else {
                0.0
            };

        velocity.y =
            if projectile.direction == FacingDirection::Up {
                1.0 * TIME_STEP * BASE_PROJECTILE_SPEED
            } else if projectile.direction == FacingDirection::Down {
                -1.0 * TIME_STEP * BASE_PROJECTILE_SPEED
            } else {
                0.0
            };
    }
}
