#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

use bevy::ui::widget::UiImageSize;
use bevy_rapier2d::prelude::*;
use rand::Rng;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));

    app.insert_resource(RapierConfiguration {
        gravity: Vec2::new(0., -900.),
        ..RapierConfiguration::new(1.)
    });
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_systems(Startup, setup);
    app.add_systems(Update, (move_paddle, camera_follow));

    app.run();
}

#[derive(Component)]
struct MovementController {
    intent: Vec2,
    max_speed: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..10 {
        commands.spawn((
            Name::new("Player1"),
            SpriteBundle {
                texture: asset_server.load("ducky.png"),
                ..Default::default()
            },
            MovementController {
                max_speed: 1.,
                intent: Vec2::ZERO,
            },
            RigidBody::Dynamic,
            Collider::cuboid(16.0, 16.0),
            ExternalForce::default(),
            ExternalImpulse::default(),
            Restitution::coefficient(1.0),
        ));
    }

    commands.spawn((
        Name::new("Player1"),
        SpriteBundle {
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        MovementController {
            max_speed: 1.,
            intent: Vec2::ZERO,
        },
        RigidBody::Dynamic,
        Collider::cuboid(16.0, 16.0),
        ExternalForce::default(),
        ExternalImpulse::default(),
        Restitution::coefficient(1.0),
    ));

    commands.spawn((
        Name::new("Floor"),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., -300., 0.)),
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(800.0, 16.0),
    ));
    commands.spawn((
        Name::new("Wall1"),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-600., 0., 0.)),
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(16.0, 400.0),
    ));
    commands.spawn((
        Name::new("Wall2"),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(600., 0., 0.)),
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(16.0, 400.0),
    ));
    commands.spawn((
        Name::new("Wall3"),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 300., 0.)),
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(800.0, 16.0),
    ));
}

fn move_paddle(
    mut paddles: Query<(&mut ExternalImpulse, &MovementController)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(KeyCode::KeyW) {
            pos.impulse = Vec2::new(0., 100000.);
        }

        if input.pressed(KeyCode::KeyS) {
            pos.impulse = Vec2::new(0., -100000.);
        }

        if input.pressed(KeyCode::KeyA) {
            pos.impulse = Vec2::new(-100000., 0.);
        }

        if input.pressed(KeyCode::KeyD) {
            pos.impulse = Vec2::new(100000., 0.);
        }
    }
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, With<MovementController>>,
) {
    let mut average_position = Vec3::ZERO;
    let mut count = 0;

    for transform in player_query.iter() {
        average_position += transform.translation;
        count += 1;
    }

    if count > 0 {
        average_position /= count as f32;
    }

    for mut camera_transform in camera_query.iter_mut() {
        camera_transform.translation = average_position;
    }
}
