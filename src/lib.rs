use app::MyAppPlugin;
use bevy::prelude::*;

mod app;
mod games;
mod inspector;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MyAppPlugin)
        .run();
}
