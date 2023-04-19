use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy::window::PrimaryWindow;

pub const WINDOW_WIDTH: f32 = 500.0;
pub const WINDOW_HEIGHT: f32 = 500.0;

pub const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const PLAYER_SIZE: Vec3 = Vec3::new(GRID_SIZE, GRID_SIZE, 10.0);

pub const PLAYER_SPEED: f32 = 100.0;
pub const GRID_SIZE: f32 = 50.0;

pub const CELL_X_COUNT: u32 = 10;
pub const CELL_Y_COUNT: u32 = 10;

fn main() {
    App::new()
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "move towards!".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(DebugLinesPlugin::default())
        .add_system(draw_grid)

        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)

        .add_system(move_player)
        .add_system(set_player_move_direction)
        .run();
}

#[derive(Component)]
pub struct Player{}

#[derive(Component)]
pub struct MoveInfo {
    direction: Vec3,
    to_position: Vec3,
}

pub fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_player(mut commands: Commands){
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            transform: Transform {
                scale: PLAYER_SIZE,
                ..default()
            },
            ..default()
        },
        Player{},
    ));
}

pub fn draw_grid(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut lines: ResMut<DebugLines>
) {
    let window = window_query.get_single().unwrap();
    let half_win_width = 0.5 * window.width();
    let half_win_height = 0.5 * window.height();
    let x_space = window.width() / CELL_X_COUNT as f32;
    let y_space = window.height() / CELL_Y_COUNT as f32;

    let mut i = -1. * half_win_height;
    while i < half_win_height {
        lines.line(
            Vec3::new(-1. * half_win_width, i, 0.0),
            Vec3::new(half_win_width, i, 0.0),
            0.0,
        );
        i += y_space;
    }

    i = -1. * half_win_width;
    while i < half_win_width {
        lines.line(
            Vec3::new(i, -1. * half_win_height, 0.0),
            Vec3::new(i, half_win_height, 0.0),
            0.0,
        );
        i += x_space;
    }

    lines.line(
        Vec3::new(0., -1. * half_win_height, 0.0),
        Vec3::new(0., half_win_height, 0.0),
        0.0,
    );
}

pub fn set_player_move_direction (
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &Transform), (With<Player>, Without<MoveInfo>)>,
) {

    if let Ok((entity, transform)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        if direction.length() > 0.0 {
            commands.entity(entity).insert(
                MoveInfo {
                    direction: direction,
                    to_position: transform.translation + GRID_SIZE * direction,
                }
            );
        }
    }
}

pub fn move_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &MoveInfo, &mut Transform), With<Player>>,
    time: Res<Time>,
){
    if let Ok((entity, move_info,  mut transform) ) = player_query.get_single_mut() {
        transform.translation += move_info.direction * PLAYER_SPEED * time.delta_seconds();

        let difference = move_info.to_position - transform.translation;
        let dot = move_info.direction.dot(difference);

        if dot < 0.0 {
            transform.translation = move_info.to_position;
            commands.entity(entity).remove::<MoveInfo>();
        }
    }
}
