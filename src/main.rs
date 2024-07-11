mod map;
mod player;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
}

use prelude::*;

struct State {
    // map: Map,
    map_builder: MapBuilder,
    player: Player,
}

impl State {
    fn new() -> Self {
        let map = Map::new();
        Self {
            // map,
            map_builder: MapBuilder::new(&mut RandomNumberGenerator::new()),
            player: Player::new(Point { x: 10, y: 10 }),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
          self.player.update(ctx, &self.map_builder.map);
        self.map_builder.map.render(ctx);
        self.player.render(ctx)
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .build()?;
    let mut state = State::new();
    // state.map.tiles[map_index(SCREEN_WIDTH/2, SCREEN_HEIGHT/2)] = TileType::Wall;
    main_loop(context, state)
}
