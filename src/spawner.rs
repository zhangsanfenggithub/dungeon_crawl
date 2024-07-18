pub use crate::prelude::*;


pub fn spawn_player(ecs: &mut World, pos: Point) {
    //添加一个entity实体, Player作为标签, 包含了pos, Render
    ecs.push((Player, pos, Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('@'),
    }));
}


pub fn spawn_enemy(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push((Enemy, pos, Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: match rng.range(0, 4) {
            0 => to_cp437('E'),
            1 => to_cp437('O'),
            2 => to_cp437('o'),
            _ => to_cp437('g')
        },

    }, MovingRandomly {}));
}