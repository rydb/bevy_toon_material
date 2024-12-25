use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_toon_material::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ToonShaderPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut toon: ResMut<Assets<ToonMaterial>>,
    mut standard: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // spawn camera
    cmd.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(0.0, 40.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
        ToonCamera,
    ));

    let toon_mat_black = toon.add(ToonMaterial {
        base_color: Color::linear_rgb(0.0, 0.0, 0.0).into(),
        ambient_color: Color::linear_rgba(0.1, 0.1, 0.1, 1.0).into(),
        texture: Some(images.add(uv_debug_texture())),
        band_count: 0,
        ..default()
    });

    let toon_mat_gray = toon.add(ToonMaterial {
        base_color: Color::linear_rgb(0.66, 0.05, 0.9).into(),
        ambient_color: Color::linear_rgba(0.1, 0.1, 0.1, 1.0).into(),
        texture: Some(images.add(uv_debug_texture())),
        band_count: 3,
        ..default()
    });

    let toon_mat_default = toon.add(ToonMaterial {
        texture: Some(images.add(uv_debug_texture())),
        ..Default::default()
    });

    // spawn spheres to demonstrate material with different colors
    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(5.0).mesh().uv(32, 18))),
        MeshMaterial3d(toon_mat_black.clone()),
        Transform::from_xyz(-15.0, 0.0, 0.0),
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(5.0).mesh().uv(32, 18))),
        MeshMaterial3d(toon_mat_gray.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(5.0).mesh().uv(32, 18))),
        MeshMaterial3d(toon_mat_default.clone()),
        Transform::from_xyz(15.0, 0.0, 0.0),
    ));

    // spawn light that affects shader
    cmd.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-20.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ToonLight,
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Cuboid::new(100.0, 1.0, 100.0))),
        MeshMaterial3d(standard.add(StandardMaterial {
            base_color: Color::LinearRgba(LinearRgba::BLACK),
            ..default()
        })),
        Transform::from_xyz(0.0, -10.0, 0.0),
    ));
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
