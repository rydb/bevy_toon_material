use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType};

const TOON_SHADER_LOCATION: &str = "../src/toon.wgsl";

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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[uniform(0, ToonShaderUniform)]
pub struct ToonShader {
    pub base_color: LinearRgba,
    pub light_direction: Vec3,
    pub light_color: LinearRgba,
    pub camera_position: Vec3,
    pub ambient_color: LinearRgba,
}

#[derive(Default, Clone, ShaderType)]
pub struct ToonShaderUniform {
    pub base_color: LinearRgba,
    pub light_direction: Vec3,
    pub light_color: LinearRgba,
    pub camera_position: Vec3,
    pub ambient_color: LinearRgba,
}

impl Material for ToonShader {
    fn fragment_shader() -> ShaderRef {
        TOON_SHADER_LOCATION.into()
    }
}

impl Default for ToonShader {
    fn default() -> Self {
        ToonShader {
            base_color: LinearRgba::WHITE,
            light_direction: Vec3::ZERO,
            light_color: LinearRgba::WHITE,
            camera_position: Vec3::ZERO,
            ambient_color: LinearRgba::new(0.4, 0.4, 0.4, 1.0),
        }
    }
}

impl AsBindGroupShaderType<ToonShaderUniform> for ToonShader {
    fn as_bind_group_shader_type(
        &self,
        _: &bevy::render::render_asset::RenderAssets<bevy::render::texture::GpuImage>,
    ) -> ToonShaderUniform {
        ToonShaderUniform {
            base_color: self.base_color,
            light_direction: self.light_direction,
            light_color: self.light_color,
            camera_position: self.camera_position,
            ambient_color: self.ambient_color,
        }
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
