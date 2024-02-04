mod map;
mod utils;
mod constants;
mod draw;
mod game_state;
mod components;
mod systems;
mod game_tick_ctx;
mod texture_cache;

use std::env;
use std::time::Duration;
use sdl2::event::{Event, WindowEvent};
use sdl2::pixels::Color;
use specs::{Builder, World, WorldExt};
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::renderable::Renderable;
use crate::components::viewshed::Viewshed;
use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use crate::game_state::GameState;
use crate::game_tick_ctx::GameTickCtx;
use crate::map::Map;
use crate::texture_cache::TextureCache;
use crate::utils::calc_tile_size;

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

    // init SDL
    let sdl_context = sdl2::init()?;
    // init SDL TTF
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = env::var("RS_GAME_FONT_PATH").map_err(|e| format!("{}: {}", e.to_string(), "RS_GAME_FONT_PATH"))?;
    let font = ttf_context.load_font(font_path, 14)?;

    let video_subsystem = sdl_context.video()?;
    let pix_w = 800;
    let pix_h = 600;

    let window = video_subsystem
        .window("rust-sdl2 demo: Events", pix_w, pix_h)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    // generate font texture.
    let texture_cache = TextureCache::init(&font, &texture_creator);

    let mut tile_size: (u32, u32) = calc_tile_size(pix_w, pix_h, TILE_WIDTH, TILE_HEIGHT);
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let event_opt = event_pump.poll_event();
        let (tile_size_w, tile_size_h) = match event_opt {
            Some(Event::Window { win_event: WindowEvent::Resized(w, h), .. }) => {
                println!("resized, w = {}, h = {}", w, h);
                tile_size = calc_tile_size(w as u32, h as u32, TILE_WIDTH, TILE_HEIGHT);
                println!("new tile_size: {:?}", tile_size);
                tile_size
            }
            _ => tile_size
        };
        let mut tick_ctx = GameTickCtx {
            event: &event_opt,
            canvas: &mut canvas,
            texture_cache: &texture_cache,
            tile_size_w,
            tile_size_h,
        };
        if let Some(-1) = gs.next_tick(&mut tick_ctx) {
            break 'running;
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    Ok(())
}