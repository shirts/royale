use crate::{
    Game,
    CHAR_STARTING_LOCATION
};
use crate::{
    FacingDirection,
    Location,
    Velocity,
    VelocityTrait
};

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system);
    }
}

#[derive(Component, Copy, Clone, Default)]
pub struct Player {
    location: Location,
    entity: Option<Entity>,
    direction: FacingDirection
}

impl Player {
    pub fn location(&self) -> Location {
        self.location
    }

    pub fn set_location(&mut self, x: Option<f32>, y: Option<f32>) {
        if let Some(x) = x {
            self.location.x = x;
        }

        if let Some(y) = y {
            self.location.y = y;
        }
    }

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
        .insert(game.player)
        .id()
    );
}
