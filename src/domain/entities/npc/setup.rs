use crate::domain::entities::npc::spawn::entity_npc_spawn;
use bevy::prelude::*;
use rand::{Rng, rng};

#[derive(Component)]
pub struct Npc;

pub fn setup_npcs(commands: Commands) {
    let n = 0..rng().random_range(0..100);
    entity_npc_spawn(commands, n.end);
}
