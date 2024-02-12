use specs::{Join, ReadStorage, System};
use crate::components::monster::Monster;
use crate::components::position::Position;
use crate::components::viewshed::Viewshed;

pub struct MonsterAiSystem {}

impl<'a> System<'a> for MonsterAiSystem {
    type SystemData = (ReadStorage<'a, Viewshed>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Monster>);

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, pos, monster) = data;

        for (viewshed, pos, _monster) in (&viewshed, &pos, &monster).join() {
            println!("Monster considers their own existence");
        }
    }
}