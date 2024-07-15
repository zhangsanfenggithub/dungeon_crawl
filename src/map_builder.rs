use std::cmp::max;
use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start_point: Point,
}


impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut map_builder = Self {
            map: Map::new(),
            rooms: Vec::new(),
            player_start_point: Point::zero(),
        };

        map_builder.fill(TileType::Wall);
        map_builder.build_random_rooms(rng);
        map_builder.build_corridors(rng);
        map_builder.player_start_point = map_builder.rooms[0].center();
        map_builder
    }

    fn fill(&mut self, tile_type: TileType) {
        self.map.tiles.iter_mut().for_each(|t| { *t = tile_type })
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            //select area
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            let mut overlap: bool = false;
            for existed_room in self.rooms.iter() {
                if existed_room.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|point| {
                    if point.x > 0 && point.x < SCREEN_WIDTH && point.y > 0 && point.y < SCREEN_HEIGHT {
                        let index = map_index(point.x, point.y);
                        self.map.tiles[index] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index] = TileType::Floor
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index] = TileType::Floor
            }
        }
    }

    pub fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| {

            a.center().x.cmp(&b.center().x)
        });
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let curr = rooms[i].center();
            self.apply_horizontal_tunnel(prev.x, curr.x, prev.y);
            self.apply_vertical_tunnel(prev.y, curr.y, curr.x);
        }
    }
}