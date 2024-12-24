use bevy::prelude::*;
use bevy_toon_material::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ToonShaderPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut toon: ResMut<Assets<ToonShader>>,
    mut standard: ResMut<Assets<StandardMaterial>>,
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

    let toon_mat_r = toon.add(ToonShader {
        base_color: Color::linear_rgb(0.9, 0.05, 0.05).into(),
        ambient_color: Color::linear_rgba(0.1, 0.1, 0.1, 1.0).into(),
        band_count: 0,
        ..default()
    });

    let toon_mat_g = toon.add(ToonShader {
        base_color: Color::linear_rgb(0.05, 0.9, 0.05).into(),
        ambient_color: Color::linear_rgba(0.1, 0.1, 0.1, 1.0).into(),
        band_count: 2,
        ..default()
    });

    let toon_mat_b = toon.add(ToonShader {
        base_color: Color::linear_rgb(0.05, 0.05, 0.9).into(),
        ambient_color: Color::linear_rgba(0.1, 0.1, 0.1, 1.0).into(),
        band_count: 3,
        ..default()
    });

    // spawn spheres to demonstrate material with different colors
    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(5.0))),
        MeshMaterial3d(toon_mat_r.clone()),
        Transform::from_xyz(-15.0, 0.0, 0.0),
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(5.0))),
        MeshMaterial3d(toon_mat_g.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(5.0))),
        MeshMaterial3d(toon_mat_b.clone()),
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
