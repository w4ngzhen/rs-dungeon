use std::cmp::{max, min};
use ggez::{Context, GameError, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::input::keyboard::{KeyCode, KeyInput};
use specs::{Join, RunNow, World, WorldExt};
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::renderable::Renderable;
use crate::components::viewshed::Viewshed;
use crate::constants::tile::TileType;
use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::draw::draw_map::draw_map;
use crate::draw::draw_renderable::draw_renderable;
use crate::game_context::GameContext;
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

    fn calc_tile_size(&self, canvas_size: (u32, u32)) -> (f32, f32) {
        let (w, h) = canvas_size;
        (w as f32 / TILE_WIDTH as f32, h as f32 / TILE_HEIGHT as f32)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.run_systems();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("FPS: {:?}", ctx.time.fps());
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let physical_size = ctx.gfx.window().inner_size();
        let (tile_size_w, tile_size_h) = self.calc_tile_size((physical_size.width, physical_size.height));
        // Draw code here...
        // draw.
        let mut game_ctx = GameContext {
            canvas: &mut canvas,
            tile_size_w,
            tile_size_h,
        };
        draw_map(&self.ecs, &mut game_ctx);
        // draw all renderable things.
        let position_store = self.ecs.read_storage::<Position>();
        let renderable_store = self.ecs.read_storage::<Renderable>();
        for (pos, renderable) in (&position_store, &renderable_store).join() {
            draw_renderable(pos, renderable, &mut game_ctx);
        }
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        if let Some(ref keycode) = input.keycode {
            player_input(self, keycode);
        }
        Ok(())
    }
}

fn player_input(gs: &mut GameState, keycode: &KeyCode) {
    // Player movement
    match keycode {
        KeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
        KeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
        KeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
        KeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
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