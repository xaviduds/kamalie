use crate::{
    application::util::rand::{util_random_2d_size, util_random_color, util_random_position},
    domain::entities::npc::setup::Npc,
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

pub fn entity_npc_check_spawn(
    commands: Commands,
    repeating_period_time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    if timer.0.tick(repeating_period_time.delta()).just_finished() {
        entity_npc_spawn(commands, 1);
    }
}

pub fn entity_npc_spawn(mut commands: Commands, npcs: i32) {
    for _ in 0..npcs {
        commands.spawn((
            Npc,
            Sprite::from_color(util_random_color(), util_random_2d_size()),
            util_random_position(),
        ));
    }
}
