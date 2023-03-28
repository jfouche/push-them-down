use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierConfiguration;

use crate::AppState;

mod enemy;
mod player;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugin(player::PlayerPlugin)
            .add_plugin(enemy::EnemyPlugin)
            .add_startup_system(stop_simulation)
            .add_system(start_simulation.in_schedule(OnEnter(SimulationState::Running)))
            .add_system(stop_simulation.in_schedule(OnExit(SimulationState::Running)))
            .add_system(toggle_simulation.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

fn toggle_simulation(
    keyboard: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if simulation_state.0 == SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            info!("Simulation Paused.");
        }
        if simulation_state.0 == SimulationState::Paused {
            next_simulation_state.set(SimulationState::Running);
            info!("Simulation Running.");
        }
    }
}

fn start_simulation(mut conf: ResMut<RapierConfiguration>) {
    conf.physics_pipeline_active = true;
}

fn stop_simulation(mut conf: ResMut<RapierConfiguration>) {
    conf.physics_pipeline_active = false;
}
