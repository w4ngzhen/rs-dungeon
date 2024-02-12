use ggez::mint::Point2;
use specs::{Join, ReadStorage, System};
use crate::components::monster::Monster;
use crate::components::name::Name;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::viewshed::Viewshed;

pub struct MonsterAiSystem {}

impl<'a> System<'a> for MonsterAiSystem {
    type SystemData = (ReadStorage<'a, Player>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, Viewshed>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Monster>);
    fn run(&mut self, data: Self::SystemData) {
        let (player_store,
            name_store,
            viewshed_store,
            pos_store,
            monster_store
        ) = data;
        for (name, viewshed, _pos, _monster) in (&name_store, &viewshed_store, &pos_store, &monster_store).join() {
            // println!("Monster considers their own existence");
            for (player_pos, _) in (&pos_store, &player_store).join() {
                let player = Point2::from([player_pos.x, player_pos.y]);
                if viewshed.visible_tiles.contains(&player) {
                    println!("{} look at you!", name.name);
                }
            }
        }
    }
}