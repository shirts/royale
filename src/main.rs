// https://github.com/bevyengine/bevy/blob/latest/examples/games/alien_cake_addict.rs
// //! A simplified implementation of the classic game "Breakout"

use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Tile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

const FLOOR_POSITION: f32 = -350.0;
const CHAR_POSITION: f32 = FLOOR_POSITION + 15.0;


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
                translation: Vec3::new(-600.0, CHAR_POSITION, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Player);
}
