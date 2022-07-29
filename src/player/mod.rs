use crate::{Character, Game, CHAR_STARTING_LOCATION};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system);
    }
}

fn spawn_player_system(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>
) {
    // set char starting location
    game.player.location = CHAR_STARTING_LOCATION;

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
