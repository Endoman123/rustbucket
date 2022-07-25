use bevy::prelude::*;

use crate::entities::components::*;

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &MoveSpeed), With<Player>>
) {
    let (mut transform, speed) = query.single_mut();

    let speed: f32 = speed.speed as f32;

    // Normalize movement delta
    let dv = Vec2::new(
        (keyboard_input.pressed(KeyCode::D) as i32 - keyboard_input.pressed(KeyCode::A) as i32) as f32,
        (keyboard_input.pressed(KeyCode::W) as i32 - keyboard_input.pressed(KeyCode::S) as i32) as f32
        ).normalize_or_zero();

    let dt: f32 = time.delta_seconds();

    transform.translation.x += dv.x * dt * speed;
    transform.translation.y += dv.y * dt * speed;
}
