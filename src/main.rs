use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_gameboard)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

enum Peg {
    Hit,
    Miss,
}

#[derive(Component)]
struct Position {
    x: i8,
    y: i8,
}

#[derive(Component)]
struct Quadrant {
    pub peg: Option<Peg>,
}

fn setup_gameboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    for x in 0..10 {
        for y in 0..10 {
            let x_pos = x as f32 * 48.0;
            let y_pos = y as f32 * 48.0;

            commands
                .spawn()
                .insert(Quadrant { peg: None })
                .insert(Position { x, y })
                .insert_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(48.0), Val::Px(48.0)),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(x_pos),
                            top: Val::Px(y_pos),
                            ..default()
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "B",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 24.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..default()
                    });
                });
        }
    }
}
