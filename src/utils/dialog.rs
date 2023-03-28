use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub fn spawn_modal_dialog<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    font: Handle<Font>,
    title: impl Into<String>,
    spawn_children: impl FnOnce(&mut ChildBuilder),
) -> EntityCommands<'w, 's, 'a> {
    let title: String = title.into();
    let mut entity_cmd = commands.spawn(Name::new(format!("MODAL DIALOG - {}", title)));
    entity_cmd
        .insert(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(50.0), Val::Percent(50.)),
                align_self: AlignSelf::Center,
                position: UiRect::left(Val::Percent(25.)),
                padding: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // TITLE
            parent
                .spawn(Name::new(format!("TITLE BAR - {}", title)))
                .insert(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(45.)),
                        ..Default::default()
                    },
                    background_color: Color::MAROON.into(),
                    ..Default::default()
                })
                .insert(Name::new(format!("TITLE BAR - {}", title)))
                .with_children(|title_bar| {
                    title_bar
                        .spawn(
                            TextBundle::from_section(
                                title,
                                TextStyle {
                                    font,
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_style(Style {
                                // align_self: AlignSelf::Center,
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                margin: UiRect::all(Val::Px(5.)),
                                padding: UiRect::all(Val::Px(5.)),
                                ..Default::default()
                            }),
                        )
                        .insert(BackgroundColor(Color::YELLOW_GREEN));
                });
        })
        // CHILDREN
        .with_children(spawn_children);

    entity_cmd
}
