mod player_input;
mod map_render;
mod entity_render;
mod collisions;
mod end_turn;
mod random_move;

use crate::prelude::*;
use crate::systems::collisions::collisions_system;

// pub fn build_scheduler() -> Schedule {
//     Schedule::builder()
//         .add_system(player_input::player_input_system())
//         .add_system(map_render::map_render_system())
//         .add_system(entity_render::entity_render_system())
//         .add_system(collisions_system())
//         .build()
// }

pub fn build_input_schedule() -> Schedule{
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}


pub fn build_player_schedule() -> Schedule{
    Schedule::builder()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}


pub fn build_enemy_schedule() -> Schedule{
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}