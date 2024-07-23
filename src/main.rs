//! Real-time atmospheric stealth game. You play as a cat that must avoid the light while exploring
//! a house inhabited by strange light beings, using the items you find to create helpful tools.
#![warn(missing_docs)]

use bevy::{
    ecs::query,
    input::common_conditions::input_pressed,
    math::{vec2, vec3},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin},
};

fn main() {
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.build().set(WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        ..Default::default()
    }))
    .add_systems(Startup, setup)
    .add_systems(Update, movement)
    .add_event::<Moving>();

    app.run();
}

#[derive(Component)]
struct Player;

#[derive(Event)]
struct Moving;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            // TODO: make this a more fitting texture
            texture: asset_server.load("IMG_2285.png"),
            transform: Transform::from_scale(vec3(0.25, 0.25, 1.0)),
            ..default()
        },
        Player,
    ));
}

fn movement(
    mut query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut event: EventWriter<Moving>,
) {
    // TODO: make this a custom run condition instead of a check within the system
    if keys.any_pressed([KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyD, KeyCode::KeyA]) {
        let mut movement = vec2(0.0, 0.0);

        if keys.pressed(KeyCode::KeyW) {
            movement.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            movement.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }

        movement = movement.normalize_or_zero();
        movement.x *= 5.0;
        movement.y *= 5.0;

        query.get_single_mut().unwrap().translation += vec3(movement.x, movement.y, 0.0);
        event.send(Moving);
    }
}
