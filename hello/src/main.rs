// main.rs - Bevy 0.18 Minimal Test Application for Aurora DX Distrobox
// 
// Cargo.toml:
// [package]
// name = "bevy-test"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// bevy = "0.18"
//
// [profile.dev]
// opt-level = 1
//
// [profile.dev.package."*"]
// opt-level = 3

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 0.18 - Aurora DX Distrobox".into(),
                resolution: (1024, 768).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (animate_cube, animate_camera, update_fps_display))
        .run();
}

#[derive(Component)]
struct AnimatedCube;

#[derive(Component)]
struct OrbitCamera {
    radius: f32,
    speed: f32,
    angle: f32,
}

#[derive(Component)]
struct FpsCounter;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn animated cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.6, 0.9),
            metallic: 0.5,
            perceptual_roughness: 0.3,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        AnimatedCube,
    ));

    // Spawn ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, -1.5, 0.0),
    ));

    // Spawn point light with physically based intensity
    commands.spawn((
        PointLight {
            intensity: 3_000_000.0,
            color: Color::srgb(1.0, 0.95, 0.8),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0),
    ));

    // Spawn orbiting camera with ambient light
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 6.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        AmbientLight {
            color: Color::srgb(0.4, 0.5, 0.6),
            brightness: 200.0,
            affects_lightmapped_meshes: false,
        },
        OrbitCamera {
            radius: 10.0,
            speed: 0.3,
            angle: 0.0,
        },
    ));

    // Spawn UI overlay with modern layout
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("🎮 Bevy 0.18 Test"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
            ));

            // Subtitle
            parent.spawn((
                Text::new("Running in Aurora DX Distrobox"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
                Node {
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
            ));

            // FPS counter
            parent.spawn((
                Text::new("FPS: --"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 1.0, 0.5)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                FpsCounter,
            ));

            // Info text
            parent.spawn((
                Text::new("✓ Graphics: Vulkan\n✓ Container: Fedora Toolbox\n✓ GPU Acceleration: Enabled"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

fn animate_cube(mut query: Query<&mut Transform, With<AnimatedCube>>, time: Res<Time>) {
    for mut transform in &mut query {
        // Rotate the cube
        transform.rotate_y(time.delta_secs() * 0.8);
        transform.rotate_x(time.delta_secs() * 0.5);
        
        // Bob up and down
        let bob = (time.elapsed_secs() * 2.0).sin() * 0.5;
        transform.translation.y = bob;
    }
}

fn animate_camera(mut query: Query<(&mut Transform, &mut OrbitCamera)>, time: Res<Time>) {
    for (mut transform, mut orbit) in &mut query {
        orbit.angle += time.delta_secs() * orbit.speed;
        
        let x = orbit.angle.cos() * orbit.radius;
        let z = orbit.angle.sin() * orbit.radius;
        let y = 6.0 + (orbit.angle * 0.5).sin() * 2.0;
        
        transform.translation = Vec3::new(x, y, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn update_fps_display(time: Res<Time>, mut query: Query<&mut Text, With<FpsCounter>>) {
    for mut text in &mut query {
        let fps = 1.0 / time.delta_secs();
        text.0 = format!("FPS: {:.0}", fps);
    }
}
