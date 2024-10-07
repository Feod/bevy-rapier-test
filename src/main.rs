// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

use bevy::ui::widget::UiImageSize;
use bevy_rapier2d::prelude::*;
use rand::Rng;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics in web builds on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));

    //Physics:
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::new(0., -900.),
        ..RapierConfiguration::new(1.)
    });
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    // My Systems:
    app.add_systems(Startup, setup);
    app.add_systems(Update, (move_paddle));

    app.run();
}

// Components:

/// These are the movement parameters for our character controller.
/// For now, this is only used for a single player, but it could power NPCs or
/// other players as well.
#[derive(Component)]
struct MovementController {
    /// The direction the character wants to move in.
    intent: Vec2,

    /// Maximum speed in world units per second.
    /// 1 world unit = 1 pixel when using the default 2D camera and no physics
    /// engine.
    max_speed: f32,
}

// Systems:

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Camera:
    commands.spawn(Camera2dBundle::default());

    //Player:
    //For loop for 10 players
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
            Restitution::coefficient(1.0), // P2aff, P7ed8
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
        Restitution::coefficient(1.0), // P2aff, P7ed8
    ));

    //Floor:
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

/*
fn move_paddle(
    mut query: Query<&mut Transform>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut force in &mut query {
        if input.pressed(KeyCode::KeyA) {
            force.translation.x -= 100. * time.delta_seconds();
            //force.force = Vec2::new(-100., 0.);
        }

        if input.pressed(KeyCode::KeyD) {
            //force.force = Vec2::new(100., 0.);
        }

        if input.pressed(KeyCode::KeyW) {
            //force.force = Vec2::new(0., 100.);
        }

        if input.pressed(KeyCode::KeyS) {
            //force.force = Vec2::new(0., -100.);
        }
    }
}
*/

fn move_paddle(
    mut paddles: Query<(&mut ExternalImpulse, &MovementController)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(KeyCode::KeyW) {
            //pos.translation.y += 100. * time.delta_seconds();
            //pos.force = Vec2::new(0., 100000.);
            pos.impulse = Vec2::new(0., 100000.);
        }

        if input.pressed(KeyCode::KeyS) {
            //pos.translation.y -= 100. * time.delta_seconds();
            //pos.force = Vec2::new(0., -100000.);
            pos.impulse = Vec2::new(0., -100000.);
        }

        if input.pressed(KeyCode::KeyA) {
            //pos.translation.x -= 100. * time.delta_seconds();
            //pos.force = Vec2::new(-100000., 0.);
            pos.impulse = Vec2::new(-100000., 0.);
        }

        if input.pressed(KeyCode::KeyD) {
            //pos.translation.x += 100. * time.delta_seconds();
            //pos.force = Vec2::new(100000., 0.);
            pos.impulse = Vec2::new(100000., 0.);
        }
    }
}

/*
fn move_paddle(
    mut players: Query<(&mut Transform, &MovementController)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (exterForce, movC) in &mut players {
        //Zero velocity
        //veloc.linear = Vec2::ZERO;

        if input.pressed(KeyCode::KeyA) {
            exterForce.translation.x -= 100. * time.delta_seconds();
            //exterForce.force = Vec2::new(-1., 0.);
        }

        if input.pressed(KeyCode::KeyD) {
            exterForce.translation.x += 100. * time.delta_seconds();
            //exterForce.force = Vec2::new(1., 0.);
        }
    }
}
*/
