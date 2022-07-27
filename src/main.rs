// https://github.com/bevyengine/bevy/blob/latest/examples/games/alien_cake_addict.rs
// //! A simplified implementation of the classic game "Breakout"

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

struct Location {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Player {
    location: Location
}

impl Player {
    fn new() -> Self {
        Self {
            location: CHAR_STARTING_LOCATION
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

// Add the game's entities to our world
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    // spawn character
    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("textures/rpg/chars/hat-guy/hat-guy.png"),
            transform: Transform {
                translation: Vec3::new(CHAR_STARTING_LOCATION.x, CHAR_STARTING_LOCATION.y, CHAR_STARTING_LOCATION.z),
                ..default()
            },
            ..default()
        })
        .insert(Player::new());
}

