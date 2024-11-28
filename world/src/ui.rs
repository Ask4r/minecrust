use bevy::prelude::*;

pub struct UIPligin;

impl Plugin for UIPligin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor);
    }
}

fn spawn_cursor(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "+",
                    TextStyle {
                        font_size: 64.0,
                        color: Color::srgb(0.4, 0.4, 0.4),
                        ..default()
                    },
                ),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
        });
}
