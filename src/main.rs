mod map;
// mod player;
mod map_builder;
mod camera;
mod systems;
mod components;
mod spawner;
mod turn_state;

mod prelude {
    //lib
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    //custom
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::systems::*;
    pub use crate::components::*;
    pub use crate::spawner::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use log::error;
use prelude::*;
use crate::turn_state::TurnState;

struct State {
    ecs: World,
    resources: Resources,
    input_scheduler: Schedule,
    player_scheduler: Schedule,
    enemy_scheduler: Schedule,
}

//State 整个世界相关状态的地方
impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng); //组合模式，组合完成后把所有权转移给构造的State
        spawn_player(&mut ecs, map_builder.player_start_point);
        map_builder.rooms.iter()
            .skip(1)
            .map(|rect| rect.center())
            .for_each(|point| spawn_enemy(&mut ecs, &mut rng, point));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start_point));
        resources.insert(TurnState::AwaitingInput);
        Self {
            ecs,
            resources,
            input_scheduler: build_input_schedule(),
            player_scheduler: build_player_schedule(),
            enemy_scheduler: build_enemy_schedule(),
        }
    }
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        let curr_state = self.resources.get::<TurnState>().unwrap().clone();
        match curr_state {
            TurnState::AwaitingInput => self.input_scheduler.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_scheduler.execute(&mut self.ecs, &mut self.resources),
            TurnState::EnemyTurn => self.enemy_scheduler.execute(&mut self.ecs, &mut self.resources),
        }
        // self.input_systems.execute(&mut self.ecs, &mut self.resources);
        // println!("current state is : {:?}", curr_state);
        render_draw_buffer(ctx).expect("Render Error")
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;
    let state = State::new();
    // state.map.tiles[map_index(SCREEN_WIDTH/2, SCREEN_HEIGHT/2)] = TileType::Wall;
    main_loop(context, state)
}
