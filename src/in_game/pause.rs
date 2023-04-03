use bevy::prelude::*;

use crate::{ui::dialog::spawn_modal_dialog, UiFont};

use super::SimulationState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_menu.in_schedule(OnEnter(SimulationState::Paused)))
            .add_system(despawn_menu.in_schedule(OnExit(SimulationState::Paused)));
    }
}

#[derive(Component)]
struct PauseMenu;

fn spawn_menu(mut commands: Commands, font: Res<UiFont>) {
    spawn_modal_dialog(&mut commands, font.clone(), "Pause menu", |dialog| {
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 20.0,
            color: Color::WHITE,
        };
        dialog.spawn(TextBundle {
            text: Text::from_sections([
                TextSection::new("PAUSE MENU", text_style.clone()),
                TextSection::from_style(text_style),
            ]),
            style: Style {
                size: Size::new(Val::Percent(90.0), Val::Auto),
                margin: UiRect::all(Val::Px(5.)),
                padding: UiRect::all(Val::Px(5.)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        });
    })
    .insert(PauseMenu);
}

fn despawn_menu(mut commands: Commands, menu: Query<Entity, With<PauseMenu>>) {
    if let Ok(entity) = menu.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
