use bevy::prelude::*;

use super::resource::{DefaultSceneAssets, DungeonSceneAssets};

pub fn remove_default_res(mut commands: Commands) {
    commands.remove_resource::<DefaultSceneAssets>();
}

pub fn remove_dungeon_res(mut commands: Commands) {
    commands.remove_resource::<DungeonSceneAssets>();
}
