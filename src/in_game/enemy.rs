use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{in_game::player::Player, AppState};

use super::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawningConfig>()
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::InGame)))
            .add_systems(
                (spawn_enemy, move_enemies)
                    .in_set(OnUpdate(AppState::InGame))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}

const ENEMY_SPAWN_TIMESTEP: f32 = 5.;
const FORCE: f32 = 20.0;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct EnemySpawningConfig {
    timer: Timer,
}

impl Default for EnemySpawningConfig {
    fn default() -> Self {
        EnemySpawningConfig {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIMESTEP, TimerMode::Repeating),
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut config: ResMut<EnemySpawningConfig>,
) {
    config.timer.tick(time.delta());
    if config.timer.finished() {
        info!("spawn_enemy(...)");

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
}

fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
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
