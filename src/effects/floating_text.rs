use bevy::prelude::*;
use crate::components::{FloatingText, InGame};

pub fn spawn_floating_text(
    commands: &mut Commands,
    text: &str,
    position: Vec2,
    color: Color,
    font_size: f32,
    lifetime: f32,
    rise_speed: f32,
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font_size,
                    color,
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(position.x, position.y, 60.0),
            ..default()
        },
        FloatingText { lifetime, max_lifetime: lifetime, rise_speed },
        InGame,
    ));
}

pub fn update_floating_text(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut FloatingText, &mut Text)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut tf, mut ft, mut text) in &mut query {
        tf.translation.y += ft.rise_speed * dt;
        ft.lifetime -= dt;

        let alpha = (ft.lifetime / ft.max_lifetime).clamp(0.0, 1.0);
        if let Some(section) = text.sections.first_mut() {
            let c = section.style.color.to_srgba();
            section.style.color = Color::srgba(c.red, c.green, c.blue, alpha);
        }

        if ft.lifetime <= 0.0 {
            if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
        }
    }
}
