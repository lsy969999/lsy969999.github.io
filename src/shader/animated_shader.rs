use bevy::{prelude::*, render::render_resource::AsBindGroup};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct AnimatedShader {}

impl Material for AnimatedShader {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}
