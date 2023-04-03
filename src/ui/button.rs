use bevy::prelude::*;

const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

const LABEL_STYLE: Style = Style { ..Style::DEFAULT };

// #[bundle]
// pub struct MyButtonBundle {
//     #[bundle]
//     button_bundle: ButtonBundle,
// }
