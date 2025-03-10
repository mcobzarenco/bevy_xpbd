//! This example is a version of Bevy's 3d_shapes example that uses trimesh colliders for the shapes.
//!
//! You could also use convex decomposition to generate compound shapes from Bevy meshes.
//! The decomposition algorithm can be slow, but the generated colliders are often faster and more robust
//! than trimesh colliders.

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_xpbd_3d::{math::*, prelude::*};
use examples_common_3d::XpbdExamplePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            XpbdExamplePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        shape::Cube::default().into(),
        shape::Box::default().into(),
        shape::Capsule::default().into(),
        shape::Torus::default().into(),
        shape::Cylinder::default().into(),
        shape::Icosphere::default().try_into().unwrap(),
        shape::UVSphere::default().into(),
    ];

    let num_shapes = shapes.len();

    // Spawn shapes
    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            RigidBody::Dynamic,
            Collider::trimesh_from_bevy_mesh(&shape).unwrap(),
            Position(Vector::new(
                -14.5 / 2.0 + i as Scalar / (num_shapes - 1) as Scalar * 14.5,
                2.0,
                0.0,
            )),
            Rotation(Quaternion::from_rotation_x(0.4)),
            PbrBundle {
                mesh: meshes.add(shape),
                material: debug_material.clone(),
                ..default()
            },
        ));
    }

    // Ground plane
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(50.0, 0.1, 50.0),
        Position(Vector::NEG_Y),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: materials.add(Color::SILVER.into()),
            ..default()
        },
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

/// Creates a colorful test pattern
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
    )
}
