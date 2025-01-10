use bevy::prelude::*;

use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 1.);

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneRoot,
    transform: Transform,
}

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn((
    //     Transform::default(),
    //     Visibility::default(),
    //     Velocity {
    //         value: Vec3::new(0., 0., 0.),
    //     },
    // ));
    // let model_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("Spaceship.glb"));
    commands.spawn(SpaceshipBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneRoot(asset_server.load("Spaceship.glb#Scene0")),
        // model: SceneRoot(model_scene),
        transform: Transform::from_translation(STARTING_TRANSLATION),
    });
}