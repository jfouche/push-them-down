use bevy::prelude::*;

use crate::{utils::dialog::spawn_modal_dialog, AppState, UiFont};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

#[derive(Component)]
struct MainMenu;

fn spawn_menu(mut commands: Commands, font: Res<UiFont>) {
    spawn_modal_dialog(&mut commands, font.clone(), "Main menu", |dialog| {
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 20.0,
            color: Color::WHITE,
        };
        dialog.spawn(TextBundle {
            text: Text::from_sections([
                TextSection::new("MAIN MENU", text_style.clone()),
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
    .insert(MainMenu);
}

fn despawn_menu(mut commands: Commands, menu: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = menu.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
