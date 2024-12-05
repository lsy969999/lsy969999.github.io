use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum FoxAssetsLoadState {
    #[default]
    Loading,
    LoadEnd,
}
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum RequiredAssetsLoadState {
    #[default]
    Loading,
    LoadEnd,
}
