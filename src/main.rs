mod entities;
mod systems;

use bevy::prelude::*;
use entities::components::*;
use systems::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(lifetime_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert_bundle(PlayerBundle {
            speed: MoveSpeed {
                speed: 420.69
            },
            sprite: SpriteBundle {
                texture: asset_server.load("test.png"),
                ..default()
            },
            ..default()
        });

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("happy.png"),
            ..default()
        })
        .insert(Health {
            max: 3_f32,
            current: 3_f32,
        })
        .insert(Decay::default());
}
