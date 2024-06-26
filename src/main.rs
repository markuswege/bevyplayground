use bevy::prelude::*;
use iyes_perf_ui::diagnostics::PerfUiEntryFPS;
use iyes_perf_ui::prelude::PerfUiEntryFPSWorst;
use iyes_perf_ui::{PerfUiPlugin, PerfUiRoot};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                sprite_movement,
                ship_movement_input,
                confine_player_to_screen,
                bullet_firing,
            ),
        )
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        PerfUiRoot {
            display_labels: true,
            layout_horizontal: false,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));

    // Spaceship
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("spaceship.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_scale(Vec3::splat(2.)),
            ..default()
        },
        SpriteMovement {
            direction: Vec3::splat(0.0),
            speed: 200.0,
        },
        CooldownTimer(Timer::from_seconds(0.1, TimerMode::Once)),
    ));
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&SpriteMovement, &mut Transform)>) {
    for (movement, mut transform) in &mut sprite_position {
        transform.translation +=
            movement.direction.normalize_or_zero() * movement.speed * time.delta_seconds();
    }
}

fn ship_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut SpriteMovement, With<Player>>,
) {
    let mut sprite_movement = player.single_mut();

    if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft)
    {
        sprite_movement.direction.x = -1.0;
    } else if (keyboard_input.just_released(KeyCode::KeyA)
        || keyboard_input.just_released(KeyCode::ArrowLeft))
        && sprite_movement.direction.x < 0.0
    {
        sprite_movement.direction.x = 0.0;
    }

    if keyboard_input.just_pressed(KeyCode::KeyD)
        || keyboard_input.just_pressed(KeyCode::ArrowRight)
    {
        sprite_movement.direction.x = 1.0;
    } else if (keyboard_input.just_released(KeyCode::KeyD)
        || keyboard_input.just_released(KeyCode::ArrowRight))
        && sprite_movement.direction.x > 0.0
    {
        sprite_movement.direction.x = 0.0;
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
        sprite_movement.direction.y = 1.0;
    } else if (keyboard_input.just_released(KeyCode::KeyW)
        || keyboard_input.just_released(KeyCode::ArrowUp))
        && sprite_movement.direction.y > 0.0
    {
        sprite_movement.direction.y = 0.0;
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown)
    {
        sprite_movement.direction.y = -1.0;
    } else if (keyboard_input.just_released(KeyCode::KeyS)
        || keyboard_input.just_released(KeyCode::ArrowDown))
        && sprite_movement.direction.y < 0.0
    {
        sprite_movement.direction.y = 0.0;
    }
}

fn confine_player_to_screen(
    mut player: Query<(&mut Transform, &mut SpriteMovement), With<Player>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let half_width = window.resolution.width() / 2.0;
    let half_height = window.resolution.height() / 2.0;

    let (mut transform, mut movement) = player.single_mut();

    if transform.translation.x < -half_width && movement.direction.x < 0.0 {
        movement.direction.x = 0.0;
        transform.translation.x = -half_width;
    } else if transform.translation.x > half_width && movement.direction.x > 0.0 {
        movement.direction.x = 0.0;
        transform.translation.x = half_width;
    }
    if transform.translation.y < -half_height && movement.direction.y < 0.0 {
        movement.direction.y = 0.0;
        transform.translation.y = -half_height;
    } else if transform.translation.y > half_height && movement.direction.y > 0.0 {
        movement.direction.y = 0.0;
        transform.translation.y = half_height;
    }
}

fn bullet_firing(
    mut cmd: Commands,
    mut player: Query<(&Transform, &mut CooldownTimer), With<Player>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    let (translation, mut timer) = player.single_mut();
    timer.tick(time.delta());
    let position = translation.translation + Vec3::new(0.0, 30.0, 0.0);

    if keyboard_input.just_pressed(KeyCode::Space) && timer.finished() {
        cmd.spawn((
            Bullet,
            SpriteBundle {
                texture: asset_server.load("bullet.png"),
                transform: Transform::from_translation(position),
                ..default()
            },
            SpriteMovement {
                direction: Vec3::Y,
                speed: 500.0,
            },
        ));
        timer.reset();
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct SpriteMovement {
    direction: Vec3,
    speed: f32,
}

#[derive(Component)]
struct Bullet;

#[derive(Component, Deref, DerefMut)]
struct CooldownTimer(Timer);
