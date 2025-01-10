mod debug;
mod movement;
mod spaceship;
mod camera;

use bevy::prelude::*;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0., 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.,
        })
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins(spaceship::SpaceShipPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}





