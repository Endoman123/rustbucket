use bevy::prelude::*;

// Player Input
#[derive(Component, Default)]
pub struct Player;

#[derive(Component)]
pub struct MoveSpeed {
    pub speed: f32
}

#[derive(Component)]
pub struct RotationSpeed {
    pub speed: f32
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub speed: MoveSpeed,

    #[bundle]
    pub sprite: SpriteBundle,

    pub _p: Player
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            speed: MoveSpeed {
                speed: 0_f32
            },
            sprite: Default::default(),
            _p: Default::default()
        }
    }
}
