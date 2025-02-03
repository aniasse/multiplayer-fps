use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 0.3;
pub const SHOOT_COOLDOWN: f32 = 0.5;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct OtherPlayer {
    pub name: String,
}

#[derive(Component)]
pub struct Bullet {
    pub lifetime: Timer,
}

pub fn update_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &mut Bullet)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut bullet) in bullets.iter_mut() {
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            let new_translation = transform.translation + transform.forward() * 50.0 * time.delta_seconds();
            *transform = Transform::from_translation(new_translation).with_rotation(transform.rotation);
        }
    }
}   