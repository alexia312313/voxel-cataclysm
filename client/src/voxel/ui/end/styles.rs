use bevy::prelude::*;


pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);


pub const SUPER_UI: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    ..Style::DEFAULT
};

pub const GAME_OVER_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::End,
    size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
    ..Style::DEFAULT
};



pub const SCORE_BOX_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
    margin: UiRect::new(Val::Px(00.0), Val::Px(0.0), Val::Percent(5.0), Val::Px(0.0)),

    ..Style::DEFAULT
};


pub const TIME_BOX_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
    ..Style::DEFAULT
};


pub const QUIT_BUTTON_STYLE: Style = Style {
    margin: UiRect::new(Val::Px(00.0), Val::Px(0.0), Val::Percent(15.0), Val::Px(0.0)),
    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

