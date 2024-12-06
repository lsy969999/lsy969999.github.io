use animated_shader::AnimatedShader;
use bevy::prelude::*;

pub mod animated_shader;

pub struct MyShaderPlugin;

impl Plugin for MyShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<AnimatedShader>::default());
    }
}
