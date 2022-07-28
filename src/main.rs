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

#[derive(Clone, Copy, Default, Debug)]
struct Location {
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

#[derive(Component)]
struct Enemy;

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

#[derive(Component, Default)]
struct Player {
    location: Location,
    entity: Option<Entity>,
    direction: FacingDirection
}

#[derive(Copy, Clone, Debug)]
enum FacingDirection {
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
    player: Player
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
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
fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    windows: Res<Windows>
) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Get window size
    let window = windows.get_primary().unwrap();
    let (_width, height) = (window.width(), window.height());

    // Spawn floor
    let bottom = -height / 2.0;

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

    // set char starting location
    game.player.location = CHAR_STARTING_LOCATION;

    game.player.direction = FacingDirection::Right;

    // Spawn enemy
    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/simplespace/enemy_B.png"),
            transform: Transform {
                translation: random_location(),
                ..default()
            },
            ..default()
        })
    .insert(Enemy);

    // Spawn player
    game.player.entity = Some(
        commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/rpg/chars/sensei/sensei.png"),
            transform: Transform {
                translation: game.player.location.to_vec3(),
                ..default()
            },
            ..default()
        })
        .insert(Character)
        .id()
    );

}

fn player_action(mut _commands: Commands, keyboard_input: Res<Input<KeyCode>>, mut game: ResMut<Game>, mut person_query: Query<&mut Transform, With<Character>>) {
    let mut moved = false;
    let mut player_transform = person_query.single_mut();

    if keyboard_input.pressed(KeyCode::Right) {
        game.player.direction = FacingDirection::Right;

        // move if player will not collide with right wall
        let new_location = game.player.location.x + TILE_MOVE_SIZE;
        if new_location < RIGHT_WALL {
            moved = true;
            game.player.location.x = new_location;
        }
    } else if keyboard_input.pressed(KeyCode::Left) {
        game.player.direction = FacingDirection::Left;

        let new_location = game.player.location.x - TILE_MOVE_SIZE;
        if new_location > LEFT_WALL {
            moved = true;
            game.player.location.x = new_location;
        }

    } else if keyboard_input.pressed(KeyCode::Up) {
        game.player.direction = FacingDirection::Up;

        let new_location = game.player.location.y + TILE_MOVE_SIZE;
        if new_location < TOP_WALL {
            moved = true;
            game.player.location.y = new_location;
        }
    } else if keyboard_input.pressed(KeyCode::Down) {
        game.player.direction = FacingDirection::Down;

        let new_location = game.player.location.y - TILE_MOVE_SIZE;
        if new_location > BOTTOM_WALL {
            moved = true;
            game.player.location.y = new_location;
        }
    }

    if moved {
        player_transform.translation.x = game.player.location.x;
        player_transform.translation.y = game.player.location.y;
        player_transform.translation.z = game.player.location.z;
    }
}

fn shoot_action(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, game: ResMut<Game>) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return
    }

    let missile_location = Location { x: game.player.location.x + 5.0, ..game.player.location };

    Some(
        commands
        .spawn()
        .insert(Projectile::new(game.player.direction))
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
