use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player::Player;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(ENEMY_SPAWN_TIMESTEP))
            .add_system(spawn_enemy.in_schedule(CoreSchedule::FixedUpdate))
            .add_system(move_enemies);
    }
}

const ENEMY_SPAWN_TIMESTEP: f32 = 5.;
const FORCE: f32 = 20.0;

#[derive(Component)]
struct Enemy;

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut images: ResMut<Assets<Image>>,
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
                transform: Transform::from_xyz(0.0, SIZE + 10.0, 0.0),
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

///
/// Moves enemies in direction of the player
///
fn move_enemies(
    mut enemies: Query<(&mut ExternalForce, &Transform), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
) {
    if let Ok(player) = player.get_single() {
        for (mut force, enemy) in enemies.iter_mut() {
            let direction = player.translation - enemy.translation;
            *force = ExternalForce::at_point(
                direction.normalize_or_zero() * FORCE,
                enemy.translation,
                enemy.translation,
            );
        }
    }
}
