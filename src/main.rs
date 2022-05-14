use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_gameboard)
        .run();
}

#[derive(Component)]
struct Quadrant {
    pub x: i8,
    pub y: i8,
    pub ship: Option<bool>,
}

fn setup_gameboard(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for x in 0..10 {
        for y in 0..10 {
            let x_pos = x as f32 * 48.0;
            let y_pos = y as f32 * 48.0;

            commands
                .spawn()
                .insert(Quadrant { x, y, ship: None })
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0., 1.0, 1.0),
                        custom_size: Some(Vec2::new(48.0, 48.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x_pos, y_pos, 1.0),
                    ..default()
                });
        }
    }
}
