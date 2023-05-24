use bevy::prelude::*;

pub const SUPER_UI: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::SpaceBetween,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    ..Style::DEFAULT
};
pub const UI: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::SpaceBetween,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
    //margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Percent(50.0), Val::Px(0.0)),
    ..Style::DEFAULT
};

pub const HEALTH_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
    margin: UiRect::new(Val::Px(60.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
    ..Style::DEFAULT
};

pub const CROSSHAIR_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    ..Style::DEFAULT
};

pub const SCORE_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
    margin: UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0)),
    ..Style::DEFAULT
};

pub const CHAT_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Start,
    align_items: AlignItems::End,
    size: Size::new(Val::Percent(15.0), Val::Percent(5.0)),
    ..Style::DEFAULT
};

pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/dogicabold.ttf"),
        font_size: 16.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}

pub fn get_text_style_title(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/dogicabold.ttf"),
        font_size: 40.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}
