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

    // pub fn render(&self, ctx: &mut BTerm, camera:& Camera) {
    //     ctx.set_active_console(0);
    //     for y in camera.top_y..camera.bottom_y {
    //         for x in camera.left_x..camera.right_x {
    //             if !self.in_bound(Point::new(x, y)) {
    //                 continue
    //             }
    //             let index = map_index(x, y);
    //             match self.tiles[index] {
    //                 TileType::Wall => {
    //                     ctx.set(x - camera.left_x, y - camera.top_y, WHITE, BLACK, to_cp437('.'))
    //                 }
    //                 TileType::Floor => {
    //                     ctx.set(x - camera.left_x, y - camera.top_y, WHITE, BLACK, to_cp437('#'))
    //                 }
    //             }
    //         }
    //     }
    // }


    pub fn in_bound(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bound(point) && self.tiles[map_index(point.x, point.y)] == TileType::Floor
    }

    ///获取tile的下标
    pub fn try_index(&self, point: Point) -> Option<usize> {
        if self.in_bound(point) {
            Some(map_index(point.x, point.y))
        } else {
            None
        }
    }
}