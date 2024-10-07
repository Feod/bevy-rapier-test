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
    app.add_systems(Update, (move_paddle, camera_follow, collect_items));

    app.run();
}

#[derive(Component)]
struct MovementController {
    intent: Vec2,
    max_speed: f32,
}

#[derive(Component)]
struct Collectible;

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
        Name::new("Floor"),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., -300., 0.)),
            texture: asset_server.load("ducky.png"),
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(800.0, 16.0),
    ));

    let wall_positions = [
        Vec3::new(-600., 0., 0.),
        Vec3::new(600., 0., 0.),
        Vec3::new(0., 300., 0.),
    ];

    for position in wall_positions.iter() {
        commands.spawn((
            Name::new("Wall"),
            SpriteBundle {
                transform: Transform::from_translation(*position),
                texture: asset_server.load("ducky.png"),
                ..Default::default()
            },
            RigidBody::Fixed,
            Collider::cuboid(16.0, 400.0),
        ));
    }

    // Spawn collectible objects
    for i in 0..5 {
        commands.spawn((
            Name::new("Collectible"),
            SpriteBundle {
                texture: asset_server.load("collectible.png"),
                transform: Transform::from_translation(Vec3::new(
                    rand::thread_rng().gen_range(-500.0..500.0),
                    rand::thread_rng().gen_range(-300.0..300.0),
                    0.0,
                )),
                ..Default::default()
            },
            Collectible,
            RigidBody::Fixed,
            Collider::cuboid(8.0, 8.0),
        ));
    }
}

fn move_paddle(
    mut paddles: Query<(&mut ExternalImpulse, &MovementController)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        let impulse_strength = 1000.0;

        if input.pressed(KeyCode::KeyW) {
            pos.impulse = Vec2::new(0., impulse_strength);
        }

        if input.pressed(KeyCode::KeyS) {
            pos.impulse = Vec2::new(0., -impulse_strength);
        }

        if input.pressed(KeyCode::KeyA) {
            pos.impulse = Vec2::new(-impulse_strength, 0.);
        }

        if input.pressed(KeyCode::KeyD) {
            pos.impulse = Vec2::new(impulse_strength, 0.);
        }
    }
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut camera_projection_query: Query<&mut OrthographicProjection, With<Camera>>,
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

    let base_zoom = 1.0;
    let zoom_level = base_zoom / (count as f32).sqrt();

    for mut camera_transform in camera_query.iter_mut() {
        camera_transform.translation = average_position;
    }

    for mut camera_projection in camera_projection_query.iter_mut() {
        camera_projection.scale = zoom_level;
    }
}

fn collect_items(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<MovementController>>,
    collectible_query: Query<(Entity, &Transform), With<Collectible>>,
) {
    for (player_entity, player_transform) in player_query.iter_mut() {
        for (collectible_entity, collectible_transform) in collectible_query.iter() {
            let distance = player_transform
                .translation
                .distance(collectible_transform.translation);
            if distance < 16.0 {
                commands.entity(collectible_entity).despawn();
            }
        }
    }
}
