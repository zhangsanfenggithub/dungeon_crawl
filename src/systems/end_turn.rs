use crate::prelude::*;
use crate::turn_state::TurnState;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let next_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
    };
    *turn_state = next_state;
}