use bevy::prelude::*;
use crate::components::{CollisionParticle, InGame};
use crate::constants::*;

pub fn spawn_collision_burst(commands: &mut Commands, position: Vec2) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for _ in 0..COLLISION_PARTICLE_COUNT {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(COLLISION_PARTICLE_SPEED * 0.5..COLLISION_PARTICLE_SPEED);
        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        let size = rng.gen_range(3.0..7.0);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.9, 0.95, 1.0, 0.8),
                    custom_size: Some(Vec2::new(size, size)),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, 30.0),
                ..default()
            },
            CollisionParticle {
                velocity,
                lifetime: COLLISION_PARTICLE_LIFETIME,
            },
            InGame,
        ));
    }
}

pub fn update_collision_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut CollisionParticle, &mut Sprite)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut tf, mut particle, mut sprite) in &mut query {
        tf.translation.x += particle.velocity.x * dt;
        tf.translation.y += particle.velocity.y * dt;
        particle.lifetime -= dt;

        // Slow down
        particle.velocity *= 1.0 - 4.0 * dt;

        let alpha = (particle.lifetime / COLLISION_PARTICLE_LIFETIME).max(0.0) * 0.8;
        sprite.color = Color::srgba(0.9, 0.95, 1.0, alpha);

        if particle.lifetime <= 0.0 {
            if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
        }
    }
}
