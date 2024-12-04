use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};

pub struct UIPligin;

impl Plugin for UIPligin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 42.0,
                    font_smoothing: default(),
                    font: default(),
                },
                // We can also change color of the overlay
                text_color: Color::srgb(0.0, 1.0, 0.0),
                enabled: true,
            },
        })
        .add_systems(Startup, spawn_cursor);
    }
}

fn spawn_cursor(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_child((
            Text::new("+"),
            TextFont {
                font_size: 64.0,
                font_smoothing: default(),
                font: default(),
            },
            TextColor(Color::srgb(0.4, 0.4, 0.4)),
            Label,
        ));
}
