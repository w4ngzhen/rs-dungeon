use sdl2::render::WindowCanvas;

pub trait GameState {
    fn next_tick(&mut self, tick_context: &TickContext) -> ();
}

pub struct TickContext<'a> {
    canvas: &'a mut WindowCanvas,
}