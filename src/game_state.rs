use std::cmp::{max, min};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use specs::{Join, RunNow, World, WorldExt};
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::renderable::Renderable;
use crate::components::viewshed::Viewshed;
use crate::constants::tile::TileType;
use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::draw::draw_map::draw_map;
use crate::game_tick_ctx::GameTickCtx;
use crate::map::Map;
use crate::systems::visibility::VisibilitySystem;
use crate::utils::xy_idx;

pub struct GameState {
    pub ecs: World,
}

impl GameState {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
    pub fn next_tick(&mut self, ctx: &mut GameTickCtx) -> Option<i32> {
        if let Some(event) = ctx.event {
            // handle input.
            match event {
                Event::Quit { .. } => {
                    return Some(-1);
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                    player_input(self, key);
                }
                _ => {
                    // nothing now
                }
            }
        }
        // run system.
        self.run_systems();
        ctx.clear(Color::RGB(255, 255, 255));
        // draw.
        draw_map(&self.ecs, ctx);
        // draw all renderable things.
        let position_store = self.ecs.read_storage::<Position>();
        let renderable_store = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&position_store, &renderable_store).join() {
            ctx.draw_text(pos.x, pos.y, render.c.to_string().as_str());
        }
        Some(0)
    }
}

fn player_input(gs: &mut GameState, keycode: &Keycode) {
    // Player movement
    match keycode {
        Keycode::Left => try_move_player(-1, 0, &mut gs.ecs),
        Keycode::Right => try_move_player(1, 0, &mut gs.ecs),
        Keycode::Up => try_move_player(0, -1, &mut gs.ecs),
        Keycode::Down => try_move_player(0, 1, &mut gs.ecs),
        _ => {}
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut position_store = ecs.write_storage::<Position>();
    let mut player_store = ecs.write_storage::<Player>();
    let mut viewshed_store = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut player_store, &mut position_store, &mut viewshed_store).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(TILE_WIDTH as i32 - 1, max(0, pos.x + delta_x));
            pos.y = min(TILE_HEIGHT as i32 - 1, max(0, pos.y + delta_y));
            // moved. we should invalid region. re-draw.
            viewshed.invalid = true;
        }
    }
}