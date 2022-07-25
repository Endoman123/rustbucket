use bevy::prelude::*;

use crate::entities::components::*;

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &MoveSpeed), With<Player>>,
) {
    let (mut transform, speed) = query.single_mut();

    let speed: f32 = speed.speed as f32;
    let dx: f32 = (keyboard_input.pressed(KeyCode::D) as i32
        - keyboard_input.pressed(KeyCode::A) as i32) as f32;
    let dy: f32 = (keyboard_input.pressed(KeyCode::W) as i32
        - keyboard_input.pressed(KeyCode::S) as i32) as f32;

    let dt: f32 = time.delta_seconds();

    // Normalize movement delta
    let dv = Vec2::new(dx, dy).normalize_or_zero();

    transform.translation.x += dv.x * dt * speed;
    transform.translation.y += dv.y * dt * speed;
}

pub fn lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (ent, mut life) in query.iter_mut() {
        life.current += time.delta_seconds();
        if life.current >= life.lifetime {
            commands.entity(ent).despawn();
        }
    }
}
