use crate::prelude::*;
use crate::turn_state::TurnState;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(ecs: &mut SubWorld,
                    command:&mut CommandBuffer,
                    #[resource] key: &Option<VirtualKeyCode>,
                    #[resource] turn_state: &mut TurnState) {
    if let Some(key) = key {
        // println!("press key : {:?}", key);
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0)
        };

        if delta.x != 0 || delta.y != 0 {
            let mut players = <(Entity, &mut Point)>::query()
                .filter(component::<Player>());
            players
                .iter_mut(ecs)
                .for_each(|(entity,pos)| {
                    let destination = *pos + delta;
                    command.push(((), WantsToMove { entity:*entity, destination }));
                });
            *turn_state = TurnState::PlayerTurn;
        }
    }
}