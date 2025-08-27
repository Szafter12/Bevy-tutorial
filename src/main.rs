use bevy::{prelude::*};

const PADDLE_START_Y: f32 = 0.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_paddle)
        .run();
}

#[derive(Component)]
struct Paddle;

fn setup (
    mut commands: Commands
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
