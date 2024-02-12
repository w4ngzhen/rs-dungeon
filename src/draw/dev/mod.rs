use ggez::Context;
use ggez::graphics::{Canvas, Color, DrawParam, Rect, Text, TextAlign, TextFragment, TextLayout};

pub fn draw_dev_info(canvas: &mut Canvas, ctx: &mut Context) {
    let mut fps_text = Text::new(TextFragment {
        text: format!("FPS: {}", ctx.time.fps()),
        color: Some(Color::RED),
        ..Default::default()
    });
    fps_text.set_layout(TextLayout {
        v_align: TextAlign::Middle,
        h_align: TextAlign::Begin,
    });
    let rect = Rect::new(0.0, 0.0, 60.0, 16.0);
    canvas.draw(&fps_text, DrawParam::new().dest(rect.center()));
}