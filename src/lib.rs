use bevy::asset::load_internal_asset;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType};

pub const TOON_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(113635888040890716522362398121248594352);

pub struct ToonShaderPlugin;

impl Plugin for ToonShaderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, TOON_SHADER_HANDLE, "toon.wgsl", Shader::from_wgsl);
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
    pub rim_amount: f32,
    pub rim_color: LinearRgba,
    pub rim_threshold: f32,
    pub band_count: u32,
}

#[derive(Default, Clone, ShaderType)]
pub struct ToonShaderUniform {
    pub base_color: LinearRgba,
    pub light_direction: Vec3,
    pub light_color: LinearRgba,
    pub camera_position: Vec3,
    pub ambient_color: LinearRgba,
    pub rim_amount: f32,
    pub rim_color: LinearRgba,
    pub rim_threshold: f32,
    pub band_count: u32,
}

impl Material for ToonShader {
    fn fragment_shader() -> ShaderRef {
        TOON_SHADER_HANDLE.into()
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
            rim_amount: 0.716,
            rim_color: LinearRgba::WHITE,
            rim_threshold: 0.1,
            band_count: 0,
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
            rim_amount: self.rim_amount,
            rim_color: self.rim_color,
            rim_threshold: self.rim_threshold,
            band_count: self.band_count,
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
