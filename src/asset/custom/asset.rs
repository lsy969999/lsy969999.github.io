use bevy::prelude::*;
use serde::Deserialize;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct CustomAsset {
    pub value: i32,
}
#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct TestAsset {
    pub test: String,
}
