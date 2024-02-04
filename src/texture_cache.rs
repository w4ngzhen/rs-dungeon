use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub struct TextureCache {}

impl TextureCache {
    pub fn init<'a>(font: &'a Font<'a, 'a>, texture_creator: &'a TextureCreator<WindowContext>) -> HashMap<String, Texture<'a>> {
        let chars: Vec<char> = vec!['.', '@', '#'];
        let mut hashmap: HashMap<String, Texture> = HashMap::new();
        for x in chars {
            let surface = font
                .render(x.to_string().as_str())
                .blended(Color::RGBA(255, 0, 0, 255))
                .unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            hashmap.insert(x.to_string(), texture);
        }
        hashmap
    }
}