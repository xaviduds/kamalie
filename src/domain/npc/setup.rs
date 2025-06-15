use crate::domain::npc::spawn::entity_npc_spawn;
use bevy::prelude::*;

#[derive(Component)]
pub struct Npc;

pub fn setup_npcs(commands: Commands) {
    entity_npc_spawn(commands);
}
