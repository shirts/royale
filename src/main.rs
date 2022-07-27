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

#[derive(Default)]
struct Location {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Character;

#[derive(Component, Default)]
struct Player {
    location: Location,
    entity: Option<Entity>,
    direction_facing: Direction
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
            .with_system(move_player)
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

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut game: ResMut<Game>, mut query: Query<&mut Transform, With<Character>>) {
    let mut moved = false;
    let mut player_transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::Right) {
        moved = true;
        game.player.location.x += TILE_MOVE_SIZE;
        game.player.direction_facing = Direction::LeftToRight;

    } else if keyboard_input.pressed(KeyCode::Left) {
        moved = true;
        game.player.location.x -= TILE_MOVE_SIZE;
        game.player.direction_facing = Direction::RightToLeft;
    } else if keyboard_input.pressed(KeyCode::Up) {
        moved = true;
        game.player.direction_facing = Direction::Inherit;
        game.player.location.y += TILE_MOVE_SIZE;
    } else if keyboard_input.pressed(KeyCode::Down) {
        moved = true;
        game.player.direction_facing = Direction::Inherit;
        game.player.location.y -= TILE_MOVE_SIZE;
    } else if keyboard_input.pressed(KeyCode::LShift) {
        match game.player.direction_facing {
            Direction::RightToLeft => {
                moved = true;
                game.player.location.x -= TILE_MOVE_SIZE * 4.0;
            },
            Direction::LeftToRight => {
                moved = true;
                game.player.location.x += TILE_MOVE_SIZE * 4.0;
            },
            _ => {}
        }
    }

    if moved {
        player_transform.translation.x = game.player.location.x;
        player_transform.translation.y = game.player.location.y;
        player_transform.translation.z = game.player.location.z;
    }
}
