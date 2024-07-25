use bevy::prelude::*;

const PADDLE_SPEED: f32 = 500.0;
const PADDLE_COLOUR: Color = Color::srgb(0.0, 0.0, 1.0);
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 100.0;

const BALL_SPEED: f32 = 300.0;
const BALL_COLOUR: Color = Color::srgb(1.0, 0.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
