use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;

use crate::AppState;

use super::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
            .add_system(despawn_player.in_schedule(OnExit(AppState::InGame)))
            .add_systems(
                (move_player, player_died)
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}

#[derive(Component)]
pub struct Player;

const FORCE: f32 = 20.0;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    const SIZE: f32 = 1.0;

    commands
        .spawn((
            Name::new("Player"),
            Player,
            PbrBundle {
                mesh: meshes.add(shape::UVSphere::default().into()),
                material: debug_material,
                transform: Transform::from_xyz(2. * SIZE, 2. * SIZE, 0.0),
                ..default()
            },
        ))
        .insert((
            RigidBody::Dynamic,
            Collider::ball(SIZE),
            ExternalForce::default(),
            Damping {
                linear_damping: 0.5,
                ..Default::default()
            },
        ));
}

fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
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

///
/// Moves the player : Up, Down, Left, Right
///
fn move_player(
    mut player: Query<(&mut ExternalForce, &Transform), With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    if let Ok((mut force, transform)) = player.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::Numpad8) {
            direction.z -= 1.;
        }
        if keys.pressed(KeyCode::Numpad2) {
            direction.z += 1.;
        }
        if keys.pressed(KeyCode::Numpad4) {
            direction.x -= 1.;
        }
        if keys.pressed(KeyCode::Numpad6) {
            direction.x += 1.;
        }
        *force = ExternalForce::at_point(
            direction.normalize_or_zero() * FORCE,
            transform.translation,
            transform.translation,
        );
    }
}

/// Check if the player died
fn player_died(mut commands: Commands, mut player: Query<(Entity, &Transform), With<Player>>) {
    if let Ok((entity, transform)) = player.get_single_mut() {
        if transform.translation.y < -20.0 {
            commands.entity(entity).despawn();
            warn!("Player died !");
        }
    }
}
