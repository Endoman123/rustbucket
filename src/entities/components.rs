use bevy::prelude::*;

// Player Input
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MoveSpeed {
    pub speed: f32
}

#[derive(Component)]
pub struct RotationSpeed {
    pub speed: f32
}
