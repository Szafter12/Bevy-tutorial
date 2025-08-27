use bevy::{prelude::*};

// Paddle
const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

// Ball
const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 400.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// Wall
const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;

const WALL_THICKNESS: f32 = 10.0;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_paddle, apply_velocity))
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider {
    size: Vec2
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: Sprite,
    collider: Collider
}

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Sprite {
            color: PADDLE_COLOR,
            custom_size: Some(PADDLE_SIZE),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(0.0, PADDLE_START_Y, 0.0 ),
            ..Default::default()
        },
        Paddle,
    ));

    let ball_texture = asset_server.load("circle.png");

    commands.spawn((
        Sprite {
            color: BALL_COLOR,
            custom_size: Some(BALL_SIZE),
            image: ball_texture,
            ..Default::default()
        },
        Transform {
            translation: BALL_STARTING_POSITION,
            ..Default::default()
        },
        Ball,
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION)
    ));
}

fn move_paddle (
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q: Query<&mut Transform, With<Paddle>>
) {

    if let Ok(mut paddle_transform) = q.single_mut() {
        let mut direction = 0.0;

        if input.pressed(KeyCode::KeyA) {
            direction -= 1.0;
        }

        if input.pressed(KeyCode::KeyD) {
            direction += 1.0;
        }

        let new_x = paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();

        paddle_transform.translation.x = new_x;
    }
}

fn apply_velocity (
    mut q: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>
) {
    let dt = time.elapsed_secs();
    for (mut transform, velocity) in &mut q {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}