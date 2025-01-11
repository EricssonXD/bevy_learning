use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const VLEOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Asteriod;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteriodPlugin;

impl Plugin for AsteriodPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_asteroid);
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector =
        || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero();

    let velocity = random_unit_vector() * VLEOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            model: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("Planet-4NxxeyYMPJ.glb")),
            ),
            transform: Transform::from_translation(translation),
        },
        Asteriod,
    ));
}
