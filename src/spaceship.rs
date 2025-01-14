use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
// const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const MISSILE_SPEED: f32 = 50.0;
#[derive(Component, Debug)]
pub struct SpaceShip;

#[derive(Component, Debug)]
pub struct SpaceShipMissile;

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, spaceship_weapoin_controls),
        );
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneRoot(asset_server.spaceship.clone()),
            transform: Transform::from_translation(STARTING_TRANSLATION),
        },
        SpaceShip,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<SpaceShip>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut movement = 0.0;
    let mut rotation = 0.0;
    let mut roll = 0.0;

    if input.pressed(KeyCode::KeyW) {
        movement += SPACESHIP_SPEED
    } else if input.pressed(KeyCode::KeyS) {
        movement -= SPACESHIP_SPEED;
    }

    if input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_secs();
    } else if input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_secs();
    }

    if input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_secs();
    } else if input.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_secs();
    }

    // We need to do this because the forward direction is the negative z-axis in the used 3D models
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll); // Barrel Roll Effect
    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapoin_controls(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
    query: Query<&Transform, With<SpaceShip>>,
) {
    let transform = query.single();
    if input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SceneRoot(scene_assets.missiles.clone()),
                transform: Transform::from_translation(transform.translation),
            },
            SpaceShipMissile,
        ));
    }
}
