use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

const TOON_SHADER_LOCATION: &str = "toon.wgsl";

pub struct ToonShaderPlugin;

impl Plugin for ToonShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ToonShader>::default())
            .add_systems(Update, update_shader);
    }
}

#[derive(Component)]
pub struct ToonLight;

#[derive(Component)]
pub struct ToonCamera;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct ToonShader {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub light_direction: Vec3,
    #[uniform(2)]
    pub light_color: LinearRgba,
    #[uniform(3)]
    pub camera_position: Vec3,
}

impl Material for ToonShader {
    fn fragment_shader() -> ShaderRef {
        TOON_SHADER_LOCATION.into()
    }
}

pub fn update_shader(
    toon_light: Single<(&DirectionalLight, &Transform), With<ToonLight>>,
    camera_position: Single<&Transform, With<ToonCamera>>,
    mut toon: ResMut<Assets<ToonShader>>,
) {
    let (light, transform) = toon_light.into_inner();
    let cam_transform = camera_position.into_inner().translation;
    for (_, toon_shader) in toon.iter_mut() {
        toon_shader.light_direction = *transform.back();
        toon_shader.light_color = light.color.to_linear();
        toon_shader.camera_position = cam_transform;
    }
}
