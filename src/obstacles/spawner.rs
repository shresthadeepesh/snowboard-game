use bevy::prelude::*;
use rand::Rng;
use crate::components::Player;
use crate::constants::*;
use crate::resources::{GameData, SpawnTimers};
use super::{tree, rock, animals};

pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<SpawnTimers>,
    game_data: Res<GameData>,
    player_query: Query<&Player>,
) {
    if player_query.get_single().map_or(true, |p| !p.intro_done) {
        return;
    }

    timers.obstacle_timer.tick(time.delta());
    if !timers.obstacle_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-WINDOW_WIDTH / 2.0 + 40.0..WINDOW_WIDTH / 2.0 - 40.0);
    let y = -WINDOW_HEIGHT / 2.0 - 80.0;
    let pos = Vec3::new(x, y, 5.0);

    let obstacle_type = if game_data.scroll_speed > 500.0 {
        rng.gen_range(0..5)
    } else if game_data.scroll_speed > 350.0 {
        rng.gen_range(0..4)
    } else {
        rng.gen_range(0..3)
    };

    match obstacle_type {
        0 => tree::spawn_tree(&mut commands, pos, &mut rng),
        1 => rock::spawn_rock(&mut commands, pos, &mut rng),
        2 => animals::spawn_deer(&mut commands, pos),
        3 => animals::spawn_rabbit(&mut commands, pos),
        _ => rock::spawn_big_rock(&mut commands, pos, &mut rng),
    }
}
