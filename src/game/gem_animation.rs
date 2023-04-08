use sdl2::{render::{WindowCanvas, Texture}, rect::Rect};

pub struct GemAnimation {
    frame_number: i32,
    is_playing: bool,
    x_offset: i32
}

impl GemAnimation {

    pub fn new(window_width: usize) -> Self {
        GemAnimation { 
            frame_number: i32::default(), 
            is_playing: bool::default(), 
            x_offset: ((window_width / 2) - 32) as i32
        }
    }

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas, texture: &Texture) {
        canvas.copy(texture, Rect::new(64 * self.frame_number, 32, 64, 64), Rect::new(self.x_offset, 20, 64, 64)).unwrap();

        if self.is_playing && self.frame_number < 8 {
            self.frame_number += 1;
        }
    }
}