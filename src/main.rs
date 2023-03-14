use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Push them down !".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_ground)
        .run();
}

fn spawn_light(mut commands: Commands) {
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
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground plane
    commands
        .spawn((
            Name::new("Ground"),
            PbrBundle {
                mesh: meshes.add(shape::Plane::from_size(50.0).into()),
                material: materials.add(Color::SILVER.into()),
                ..default()
            },
        ))
        .insert((RigidBody::Fixed, Collider::cuboid(25., 0., 25.)));
}
