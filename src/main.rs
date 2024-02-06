mod map;
mod utils;
mod constants;
mod draw;
mod game_state;
mod components;
mod systems;
mod game_context;
mod tile_pos;

use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics::Color;
use specs::{Builder, World, WorldExt};
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::renderable::Renderable;
use crate::components::viewshed::Viewshed;
use crate::game_state::GameState;
use crate::map::Map;

fn main() -> Result<(), String> {
    let mut gs = GameState {
        ecs: World::new()
    };
    // insert resource
    let map = Map::new_map();
    let first_room = &map.rooms[0];
    let (player_x, player_y) = first_room.center().to_tuple();
    gs.ecs.insert(map);
    // register
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    // create entity
    gs.ecs
        .create_entity()
        .with(Player {})
        .with(Viewshed { visible_tiles: vec![], range: 8, invalid: true })
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            c: '@',
            fg: Color::YELLOW,
            bg: Color::BLACK,
        })
        .build();
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(WindowMode::default().dimensions(1200_f32, 800_f32).resizable(true));
    let (ctx, event_loop) = cb.build().map_err(|e| e.to_string())?;
    event::run(ctx, event_loop, gs);
}