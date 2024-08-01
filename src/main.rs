use bevy::{prelude::*, window::Window};

const PADDLE_SPEED: f32 = 500.0;
const PADDLE_COLOUR: Color = Color::srgb(0.0, 0.0, 1.0);
const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 20.0;
const PADDLE_WORLD_HEIGHT: f32 = 100.0;

const BALL_SPEED: f32 = 300.0;
const BALL_COLOUR: Color = Color::srgb(1.0, 0.0, 0.0);

const BACKGROUND_COLOUR: Color = Color::srgb(0.0, 0.0, 0.0);

const GAME_WIDTH: f32 = 1280.0;
const GAME_HEIGHT: f32 = 720.0;
const ARENA_PADDING: f32 = 50.0;

const WALL_COLOUR: Color = Color::srgb(1.0, 1.0, 1.0);
const WALL_DEPTH: f32 = 10.0;

const BRICK_WIDTH: f32 = 50.0;
const BRICK_HEIGHT: f32 = 20.0;
const BRICK_COLOUR_1: Color = Color::srgb(1.0, 1.0, 0.0);
const BRICK_COLOUR_2: Color = Color::srgb(0.0, 1.0, 1.0);
const BRICK_COLOUR_3: Color = Color::srgb(1.0, 0.0, 1.0);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Brick {
    hits: u8,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

enum Axis {
    X,
    Y,
}

impl Velocity {
    fn reflect(self, axis: Axis) -> Self {
        match axis {
            Axis::X => Self {
                x: -self.x,
                y: self.y,
            },
            Axis::Y => Self {
                x: self.x,
                y: -self.y,
            },
        }
    }
}

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Breakout!".to_string(),
                resolution: (GAME_WIDTH, GAME_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (startup, spawn_paddle, spawn_ball, build_walls))
        .add_systems(
            Update,
            (
                move_paddle,
                ball_movement,
                ball_collision,
                update_sprite_position,
            )
                .chain(),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOUR))
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
                translation: Vec3::new(0.0, -GAME_HEIGHT / 2.0 + PADDLE_WORLD_HEIGHT, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Paddle)
        .insert(Position {
            x: 0.0,
            y: -GAME_HEIGHT / 2.0 + PADDLE_WORLD_HEIGHT,
        })
        .insert(Size {
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        });
}

fn spawn_ball(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: BALL_COLOUR,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Ball)
        .insert(Position { x: 0.0, y: 0.0 })
        .insert(Velocity { x: 1.0, y: 1.0 });
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
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += 1.0;
        }

        let new_x = position.x + direction * PADDLE_SPEED * time.delta_seconds();
        let min_x = -GAME_WIDTH / 2.0 + ARENA_PADDING + WALL_DEPTH / 2.0 + size.width / 2.0;
        let max_x = GAME_WIDTH / 2.0 - ARENA_PADDING - WALL_DEPTH / 2.0 - size.width / 2.0;
        //clamp doesn't allow the paddle to go beyond the wall limits
        position.x = new_x.clamp(min_x, max_x);
    }
}

fn ball_movement(time: Res<Time>, mut query: Query<(&Ball, &mut Position, &mut Velocity)>) {
    for (_, mut position, mut velocity) in query.iter_mut() {
        position.y += velocity.y * BALL_SPEED * time.delta_seconds();
        position.x += velocity.x * BALL_SPEED * time.delta_seconds();
    }
}

fn ball_collision(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &Ball, &Position, &Velocity)>,
    paddle_query: Query<(&Paddle, &Position, &Size)>,
    brick_query: Query<(&Brick, &Position, &Size)>,
) {
}

fn update_sprite_position(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

macro_rules! spawn_wall {
    ($commands: expr, $position:expr, $size:expr) => {
        $commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: WALL_COLOUR,
                    custom_size: Some($size),
                    ..default()
                },
                transform: Transform {
                    translation: $position,
                    ..default()
                },
                ..default()
            })
            .insert(Wall)
            .insert(Position {
                x: $position.x,
                y: $position.y,
            })
            .insert(Size {
                width: $size.x,
                height: $size.y,
            });
    };
}

fn build_walls(mut commands: Commands) {
    let wall_length_x = GAME_WIDTH - (ARENA_PADDING * 2.0); //GAME_WIDTH less padding on each side
    let wall_length_y = GAME_HEIGHT - (ARENA_PADDING * 2.0); //GAME_HEIGHT less padding on each side
    let wall_depth = WALL_DEPTH; // thickness of wall

    let top_wall_position = Vec3::new(
        0.0,
        GAME_HEIGHT / 2.0 - wall_depth / 2.0 - ARENA_PADDING,
        0.0,
    );
    let bottom_wall_position = Vec3::new(
        0.0,
        -GAME_HEIGHT / 2.0 + wall_depth / 2.0 + ARENA_PADDING,
        0.0,
    );
    let left_wall_position = Vec3::new(
        -GAME_WIDTH / 2.0 + ARENA_PADDING + wall_depth / 2.0,
        0.0,
        0.0,
    );
    let right_wall_position = Vec3::new(
        GAME_WIDTH / 2.0 - ARENA_PADDING - wall_depth / 2.0,
        0.0,
        0.0,
    );

    //top wall
    spawn_wall!(
        commands,
        top_wall_position,
        Vec2::new(wall_length_x, wall_depth)
    );
    //bottom wall
    spawn_wall!(
        commands,
        bottom_wall_position,
        Vec2::new(wall_length_x, wall_depth)
    );
    //left wall
    spawn_wall!(
        commands,
        left_wall_position,
        Vec2::new(wall_depth, wall_length_y)
    );
    //right wall
    spawn_wall!(
        commands,
        right_wall_position,
        Vec2::new(wall_depth, wall_length_y)
    );
}
