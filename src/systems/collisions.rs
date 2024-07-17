use crate::prelude::*;


#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();

    let mut players = <(&Point)>::query().filter(component::<Player>());
    players
        .iter(ecs)
        .for_each(|point| player_pos = *point);
    //Entity是一个类型，&Point 也是一个类型
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        .filter(|(_, pos)| player_pos == **pos)
        .for_each(|(entity, _)| commands.remove(*entity)) //传递的是Entity这个值的引用
}