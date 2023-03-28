#[cfg(test)]
mod test {
    use bevy::prelude::*;

    #[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
    enum AppState {
        #[default]
        MainMenu,
        InGame,
    }

    #[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
    enum SimulationState {
        Running,
        #[default]
        Paused,
    }

    #[test]
    fn test_multiple_states() {
        let mut app = App::new();
        app.add_state::<AppState>()
            .add_state::<SimulationState>()
            .add_system(
                spawn_pause_menu
                    .in_set(OnUpdate(AppState::InGame))
                    .in_schedule(OnEnter(SimulationState::Paused)),
            )
            .run();

        fn spawn_pause_menu(mut _commands: Commands) {
            panic!("Pause menu spawned");
        }
    }
}
