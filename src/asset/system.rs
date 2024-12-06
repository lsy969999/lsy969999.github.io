use bevy::{asset::LoadState, prelude::*};

use super::{
    event::{FontLoaded, FoxLoaded},
    resource::{
        BaseAssets, BaseAssetsLoaded, DefaultSceneAssets, DefaultSceneAssetsLoaded, MyAssets,
        MyAssetsLoaded,
    },
};

pub fn asset_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MyAssets {
        fox: asset_server.load("models/Fox.glb"),
        font: asset_server.load("fonts/GalmuriMono11.ttf"),
    });
    commands.insert_resource(MyAssetsLoaded {
        is_loaded_font: false,
        is_loaded_fox: false,
    });

    commands.insert_resource(BaseAssets {
        font: asset_server.load("fonts/GalmuriMono11.ttf"),
    });
    commands.insert_resource(BaseAssetsLoaded {
        is_loaded_font: false,
    });
    commands.insert_resource(DefaultSceneAssets {
        fox: asset_server.load("models/Fox.glb"),
    });
    commands.insert_resource(DefaultSceneAssetsLoaded {
        is_loaded_fox: false,
    });
}

pub fn on_asset_fox_load_state(
    asset_server: Res<AssetServer>,
    my_assets: Res<MyAssets>,
    mut ew_al: EventWriter<FoxLoaded>,
    mut asset_loaded: ResMut<MyAssetsLoaded>,
) {
    match asset_server.get_load_state(my_assets.fox.id()) {
        Some(LoadState::Loaded) => {
            if !asset_loaded.is_loaded_fox {
                asset_loaded.is_loaded_fox = true;
                info!("foffff");
                ew_al.send(FoxLoaded);
            }
        }
        _ => {}
    }
}

pub fn on_asset_font_load_state(
    asset_server: Res<AssetServer>,
    my_assets: Res<MyAssets>,
    mut ew_al: EventWriter<FontLoaded>,
    mut asset_loaded: ResMut<MyAssetsLoaded>,
) {
    match asset_server.get_load_state(my_assets.font.id()) {
        Some(LoadState::Loaded) => {
            if !asset_loaded.is_loaded_font {
                asset_loaded.is_loaded_font = true;
                info!("tttttt");
                ew_al.send(FontLoaded);
            }
        }
        _ => {}
    }
}
