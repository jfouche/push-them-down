use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
            .add_system(spawn_enemy.in_schedule(CoreSchedule::FixedUpdate))
            .add_system(move_enemies);
    }
}

const FIXED_TIMESTEP: f32 = 5.;

#[derive(Component)]
struct Enemy;

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _: Res<FixedTime>,
) {
    warn!("spawn_enemy(...)");

    const SIZE: f32 = 1.0;

    commands
        .spawn((
            Name::new("Enemy"),
            Enemy,
            PbrBundle {
                mesh: meshes.add(shape::UVSphere::default().into()),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..Default::default()
                }),
                transform: Transform::from_xyz(2. * SIZE, 2. * SIZE, 0.0),
                ..default()
            },
        ))
        .insert((
            RigidBody::Dynamic,
            Collider::ball(SIZE),
            ExternalImpulse::default(),
            Damping {
                linear_damping: 0.5,
                ..Default::default()
            },
        ));
}

///
/// Moves enemies
///
fn move_enemies(
    mut player: Query<(&mut ExternalImpulse, &Transform), With<Enemy>>,
    keys: Res<Input<KeyCode>>,
) {
}
