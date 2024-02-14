use ggez::mint::Point2;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use crate::components::monster::Monster;
use crate::components::name::Name;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::viewshed::Viewshed;
use pathfinding::prelude::{astar};
use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::map::Map;
use crate::utils::xy_idx;

pub struct MonsterAiSystem {}

impl<'a> System<'a> for MonsterAiSystem {
    type SystemData = (ReadExpect<'a, Map>,
                       ReadStorage<'a, Player>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, Viewshed>,
                       WriteStorage<'a, Position>,
                       ReadStorage<'a, Monster>);
    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            player_store,
            name_store,
            viewshed_store,
            mut pos_store,
            monster_store
        ) = data;
        // query player position
        let mut player_pos: Option<Position> = None;
        for (_player_pos, _) in (&pos_store, &player_store).join() {
            player_pos = Some(_player_pos.clone());
            break;
        }
        for (name, viewshed, ref mut monster_pos, _monster) in (&name_store, &viewshed_store, &mut pos_store, &monster_store).join() {
            if let Some(ref player_pos) = player_pos {
                if viewshed.visible_tiles.contains(&Point2::from([player_pos.x, player_pos.y])) {
                    // chasing player
                    println!("{} look at you!", name.name);
                    let path_nodes = calc_path_astar(monster_pos, player_pos, &map);
                    println!("path nodes: {:?}", path_nodes);
                    if path_nodes.len() >= 2 {
                        let next_pos = path_nodes.get(1).unwrap();
                        println!("next pos: {:?}", next_pos);
                        monster_pos.x = next_pos.x;
                        monster_pos.y = next_pos.y;
                    }
                }
            }
        }
    }
}

fn calc_path_astar(src: &Position, dest: &Position, map: &Map) -> Vec<Position> {
    const SCALE: u64 = 10;
    #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct Pos(u64, u64);
    impl Pos {
        fn distance(&self, other: &Pos) -> u64 {
            self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
        }
        fn successors(&self, map: &Map) -> Vec<(Pos, u64)> {
            let &Pos(curr_x, curr_y) = self;
            let next: Vec<(Pos, u64)> = vec![
                Pos(curr_x, curr_y - SCALE),
                Pos(curr_x - SCALE, curr_y),
                Pos(curr_x + SCALE, curr_y),
                Pos(curr_x, curr_y + SCALE),
            ].into_iter()
                .filter(|p| {
                    let origin_x = p.0 / SCALE;
                    let origin_y = p.1 / SCALE;
                    origin_x >= 0 && origin_x < TILE_WIDTH && origin_y >= 0 && origin_y < TILE_HEIGHT && !map.is_block(xy_idx(origin_x, origin_y))
                })
                .map(|p| {
                    let &Pos(x, y) = &p;
                    let h = x.abs_diff(curr_x);
                    let v = y.abs_diff(curr_y);
                    if v == SCALE && h == SCALE {
                        (p, 14)
                    } else if v == 0 && h == 0 {
                        (p, 0)
                    } else {
                        (p, 10)
                    }
                }).collect();
            next
        }
    }

    let src_pos = Pos(src.x * SCALE, src.y * SCALE);
    let dest_pos = Pos(dest.x * SCALE, dest.y * SCALE);
    let result = astar(
        &src_pos,
        |p| { p.successors(map) },
        |p| p.distance(&dest_pos) / 3,
        |p| *p == dest_pos,
    );
    if result == None {
        return vec![];
    }
    let res = result.unwrap();
    res.0.iter().map(|item| { Position { x: item.0 / SCALE, y: item.1 / SCALE } }).collect()
}