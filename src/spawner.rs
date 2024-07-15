pub use crate::prelude::*;


pub fn spawn_player(ecs: &mut World, pos: Point) {
    //添加一个entity实体, Player作为标签, 包含了pos, Render
    ecs.push((Player, pos, Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('@'),
    }));
}
