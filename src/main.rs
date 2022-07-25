mod systems;
mod entities;

use bevy::prelude::*;
use entities::components::*;
use systems::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("test.png"),
            ..default()
        })
        .insert(Player)
        .insert(MoveSpeed {
            speed: 420.69_f32 // nice
        });
}
