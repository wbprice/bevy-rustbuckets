use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_gameboard)
        .add_system(button_system)
        .add_system(peg_insert_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 1.);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component, Debug)]
struct Position {
    x: i8,
    y: i8,
}

#[derive(Component, Debug)]
struct Peg {
    pub hit: bool,
}

fn setup_gameboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    for x in 0..10 {
        for y in 0..10 {
            let x_pos = x as f32 * 48.0;
            let y_pos = y as f32 * 48.0;

            commands
                .spawn()
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
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(1.), Val::Px(1.)),
                            ..default()
                        },
                        ..default()
                    });
                });
        }
    }
}

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (entity, interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "P".to_string();
                *color = PRESSED_BUTTON.into();
                commands.entity(entity).insert(Peg { hit: true });
            }
            Interaction::Hovered => {
                text.sections[0].value = "H".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "B".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn peg_insert_system(mut commands: Commands, peg_insert_query: Query<Entity, Added<Peg>>) {
    for entity in peg_insert_query.iter() {
        dbg!("handle inserting peg");
        commands.entity(entity).with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                    border: Rect::all(Val::Px(2.0)),
                    ..default()
                },
                color: Color::rgb(0.65, 0.65, 0.65).into(),
                ..default()
            });
        });
    }
}
