use bevy::math::{vec2, vec3};
use bevy::sprite::collide_aabb::Collision;
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, time};

// PADDLE
const PADDLE_START_Y: f32 = BOTTOM_WALL + 60.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

// BALL
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -5.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 400.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// Wall -> Confine the ball
const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;

// Wall Properties
const WALL_THICKNESS: f32 = 10.0;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEGIHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

// Bricks
const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                apply_velocity,
                check_ball_collision.after(apply_velocity),
            ),
        )
        .run();
}

// This is a Tag component. We use this to identify the player
#[derive(Component)]
struct Paddle;

// This is a Tag component. -> BALL
#[derive(Component)]
struct Ball {
    size: Vec2,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Component)]
struct Brick;

// Wall components
#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn(Camera2dBundle::default());

    let ball_tex = asset_server.load("textures/circle.png");

    // Paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0.0, PADDLE_START_Y, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(PADDLE_SIZE),
                ..Default::default()
            },

            ..Default::default()
        },
        Paddle,
        Collider { size: PADDLE_SIZE },
    ));

    // Ball
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                ..Default::default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(BALL_SIZE),
                ..Default::default()
            },
            texture: ball_tex,
            ..Default::default()
        },
        Ball { size: BALL_SIZE },
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
    ));

    // Walls, this is a scope. we use scopes for large amounts of code
    {
        let vertical_wall_size = vec2(WALL_THICKNESS, WALL_BLOCK_HEGIHT + WALL_THICKNESS);
        let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

        // LEFT WALL
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(LEFT_WALL, 0.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider {
                size: vertical_wall_size,
            },
        });

        // RIGHT WALl
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(RIGHT_WALL, 0.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider {
                size: vertical_wall_size,
            },
        });

        // BOTTOm WALl
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, BOTTOM_WALL, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(horizontal_wall_size),
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider {
                size: horizontal_wall_size,
            },
        });

        // TOP WALL
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, TOP_WALL, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(horizontal_wall_size),
                    ..Default::default()
                },
                ..Default::default()
            },
            collider: Collider {
                size: horizontal_wall_size,
            },
        });
    }

    // BRICKS
    {
        let offset_x = LEFT_WALL + GAP_BETWEEN_BRICKS_AND_SIDES + BRICK_SIZE.x * 0.5;
        let offset_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_BRICKS + BRICK_SIZE.x * 0.5;

        let brick_total_width = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEEN_BRICKS_AND_SIDES;
        let brick_total_height = (TOP_WALL - BOTTOM_WALL)
            - GAP_BETWEEN_BRICKS_AND_CEILING
            - GAP_BETWEEN_PADDLE_AND_BRICKS;

        let rows = (brick_total_height / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as i32;
        let columns = (brick_total_width / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as i32;

        for row in 0..rows {
            for column in 0..columns {
                let brick_pos = vec2(
                    offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                    offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
                );

                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: brick_pos.extend(0.0),
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: BRICK_COLOR,
                            custom_size: Some(BRICK_SIZE),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Brick,
                    Collider { size: BRICK_SIZE },
                ));
            }
        }
    }
}

fn move_paddle(
    input: Res<Input<KeyCode>>,
    time_step: Res<FixedTime>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if input.pressed(KeyCode::A) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        direction += 1.0;
    }

    let mut new_x =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.period.as_secs_f32();

    // IMPORTNAT Do not let paddle go over the walls
    new_x = new_x.min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
    new_x = new_x.max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);

    paddle_transform.translation.x = new_x;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    let dt = time_step.period.as_secs_f32();

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}

fn check_ball_collision(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
    collider_query: Query<(Entity, &Transform, &Collider, Option<&Brick>)>,
) {
    for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
        for (other_entity, transform, other, opt_brick) in &collider_query {
            let collision = collide(
                ball_transform.translation,
                ball.size,
                transform.translation,
                other.size,
            );

            // Reflect the ball with the same velocity.
            let mut reflect_x: bool = false;
            let mut reflect_y: bool = false;

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* DO NOTHING*/ }
                }

                if reflect_x {
                    ball_velocity.x *= -1.0;
                }
                if reflect_y {
                    ball_velocity.y *= -1.0;
                }

                if opt_brick.is_some() {
                    commands.entity(other_entity).despawn();
                }
            }
        }
    }
}
