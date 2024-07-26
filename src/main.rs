use bevy::{prelude::*, ui::update};

const PADDLE_SPEED: f32 = 500.0;
const PADDLE_COLOUR: Color = Color::srgb(0.0, 0.0, 1.0);
const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 20.0;

const BALL_SPEED: f32 = 300.0;
const BALL_COLOUR: Color = Color::srgb(1.0, 0.0, 0.0);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (startup, spawn_paddle))
        .add_systems(
            Update,
            (move_paddle, update_sprite_position.after(move_paddle)),
        )
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_paddle(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: PADDLE_COLOUR,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Paddle)
        .insert(Position { x: 0.0, y: -100.0 })
        .insert(Size {
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        });
}

fn move_paddle(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Paddle, &mut Position, &Size)>,
) {
    for (_, mut position, size) in query.iter_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction -= 1.0;
            println!("Left");
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += 1.0;
            println!("Right");
        }
        position.x += direction * PADDLE_SPEED * time.delta_seconds();
        println!("Position: {}", position.x);
        println!("Time elapsed: {}", time.delta_seconds());
        position.x = position
            .x
            .min(4000.0 - size.width / 2.0)
            .max(-4000.0 + size.width / 2.0);
    }
}

fn update_sprite_position(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}
