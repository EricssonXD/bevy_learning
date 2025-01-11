use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, movement::{Acceleration, MovingObjectBundle, Velocity}};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup,spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<SceneAssets>) {
    // commands.spawn((
    //     Transform::default(),
    //     Visibility::default(),
    //     Velocity {
    //         value: Vec3::new(0., 0., 0.),
    //     },
    // ));
    commands.spawn(MovingObjectBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        acceleration: Acceleration::new(Vec3::ZERO),
        model: SceneRoot(asset_server.spaceship.clone()),
        transform: Transform::from_translation(STARTING_TRANSLATION),
    });
}