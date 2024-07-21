use crate::prelude::*;


#[system(for_each)] //Legion为止运行一条查询的系统提供的语法糖
#[read_component(Player)]
pub fn movement(
    entity: &Entity, //这两个组件就是我们要查询的组件,相当于省去手工迭代的
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer) {
    if map.can_enter_tile(want_move.destination) {
        //自动替换组件
        commands.add_component(want_move.entity, want_move.destination);
        //如果是Player，更新相机位置
        if ecs.entry_ref(want_move.entity).unwrap()
            .get_component::<Player>()
            .is_ok() {
            camera.on_player_move(want_move.destination);
        }
    }

    commands.remove(*entity)
}