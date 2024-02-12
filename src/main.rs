mod map;
mod utils;
mod constants;
mod draw;
mod game_state;
mod components;
mod systems;
mod game_context;
mod tile_pos;
mod tile_rect;
mod rand_gen;
mod run_state;

use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics::Color;
use specs::{Builder, World, WorldExt};
use crate::components::monster::Monster;
use crate::components::name::Name;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::renderable::Renderable;
use crate::components::viewshed::Viewshed;
use crate::game_state::GameState;
use crate::map::Map;
use crate::run_state::RunState;
use crate::utils::to_tuple;

fn main() -> Result<(), String> {
    let mut gs = GameState {
        ecs: World::new(),
        run_state: RunState::Running,
    };
    // register
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    // insert resource
    let map = Map::new_map();
    let first_room = &map.rooms[0];
    let (player_x, player_y) = to_tuple(first_room.center());
    // create entity
    gs.ecs
        .create_entity()
        .with(Player {})
        .with(Name { name: String::from("Tom") })
        .with(Viewshed { visible_tiles: vec![], range: 8, invalid: true })
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            c: '@',
            fg: Color::YELLOW,
            bg: Color::BLACK,
        })
        .build();
    // create monster
    let mut monster_id = 1;
    for room in map.rooms.iter().skip(1) {
        let (x, y) = to_tuple(room.center());
        gs.ecs.create_entity()
            .with(Monster {})
            .with(Name { name: format!("M#{}", monster_id) })
            .with(Position { x, y })
            .with(Renderable {
                c: 'g',
                fg: Color::RED,
                bg: Color::BLACK,
            })
            .with(Viewshed { visible_tiles: vec![], range: 8, invalid: true })
            .build();
        monster_id += 1;
    }
    gs.ecs.insert(map);
    // ggez
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(WindowMode::default().dimensions(1200_f32, 800_f32).resizable(true));
    let (ctx, event_loop) = cb.build().map_err(|e| e.to_string())?;
    event::run(ctx, event_loop, gs);
}