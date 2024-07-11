use crate::prelude::*;
//usize 依赖于平台
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}


impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = map_index(x, y);
                match self.tiles[index] {
                    TileType::Wall => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'))
                    }
                    TileType::Floor => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'))
                    }
                }
            }
        }
    }

    pub fn in_bound(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: &Point) -> bool {
        self.in_bound(point) && self.tiles[map_index(point.x, point.y)] == TileType::Floor
    }

    pub fn try_index(&self, point: &Point) -> Option<usize> {
        if self.in_bound(point) {
            Some(map_index(point.x, point.y))
        } else {
            None
        }
    }
}