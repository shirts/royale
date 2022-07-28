use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

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

const MISSILE_SIZE: Vec3 = const_vec3!([120.0, 20.0, 0.0]);
const MISSILE_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Debug)]
struct Location {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Projectile;

#[derive(Component, Default)]
struct Player {
    location: Location,
    entity: Option<Entity>,
    direction_facing: Direction
}

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

// #[derive(Component, Default)]
// struct Missile {
//     entity: Option<Entity>,
//     location: Location,
//     direction: FacingDirection
// }

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

// Add the game's entities to our world
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());


    // spawn floor
    for num in -800..800 {
        let n = num as f32;
        commands
            .spawn()
            .insert(Tile)
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("textures/rpg/tiles/generic-rpg-tile42.png"),
                transform: Transform {
                    translation: Vec3::new(n, FLOOR_POSITION, 0.0),
                    ..default()
                },
                ..default()
            });
    };

    // set char starting location
    game.player.location = CHAR_STARTING_LOCATION;

    game.player.direction_facing = Direction::RightToLeft;

    // spawn character
    game.player.entity = Some(
        commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/rpg/chars/sensei/sensei.png"),
            transform: Transform {
                translation: Vec3::new(CHAR_STARTING_LOCATION.x, CHAR_STARTING_LOCATION.y, CHAR_STARTING_LOCATION.z),
                ..default()
            },
            ..default()
        })
        .insert(Character)
        .id()
    )
}

fn player_action(mut _commands: Commands, keyboard_input: Res<Input<KeyCode>>, mut game: ResMut<Game>, mut person_query: Query<&mut Transform, With<Character>>) {
    let mut moved = false;
    let mut player_transform = person_query.single_mut();

    if keyboard_input.pressed(KeyCode::Right) {
        game.player.direction_facing = Direction::LeftToRight;

        // move if player will not collide with right wall
        let new_location = game.player.location.x + TILE_MOVE_SIZE;
        if new_location < RIGHT_WALL {
            moved = true;
            game.player.location.x = new_location;
        }
    } else if keyboard_input.pressed(KeyCode::Left) {
        game.player.direction_facing = Direction::RightToLeft;

        let new_location = game.player.location.x - TILE_MOVE_SIZE;
        if new_location > LEFT_WALL {
            moved = true;
            game.player.location.x = new_location;
        }

    } else if keyboard_input.pressed(KeyCode::Up) {
        game.player.direction_facing = Direction::Inherit;

        let new_location = game.player.location.y + TILE_MOVE_SIZE;
        if new_location < TOP_WALL {
            moved = true;
            game.player.location.y = new_location;
        }
    } else if keyboard_input.pressed(KeyCode::Down) {
        game.player.direction_facing = Direction::Inherit;

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
        .insert(Projectile)
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

fn move_missiles(mut projectile_query: Query<&mut Transform, With<Projectile>>) {
    for mut transform in projectile_query.iter_mut() {
        transform.translation.x += MISSILE_TRAVEL;
    };
}
