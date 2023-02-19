extern crate sdl2;

use sdl2::{pixels::Color, keyboard::Keycode, event::Event, image::LoadTexture, rect::Rect, render::{WindowCanvas, Texture}};
use std::{time::Duration, thread, collections::HashMap};


pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rustsweeper", 300, 400)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_ctreator = canvas.texture_creator();
    let texture = texture_ctreator.load_texture("assets/spritesheet.png").unwrap();

    let mut cell_map = HashMap::<(i32, i32), Cell>::new();
    
    for y in 0..9 {
        for x in 0..9 {
            cell_map.insert((x, y), Cell { hidden: true, is_bomb: true, adjacent_bombs: 0 });
        }
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | 
                Event:: KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event:: MouseButtonUp { x, y, ..} => {
                    handle_left_click(x, y, &mut cell_map)
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for (position, cell) in &cell_map {
            let screen_rect = Rect::new(position.0 * 32 + 6, position.1 * 32 + 50, 32, 32);

            if cell.hidden {
                let sprite_rect = Rect::new(0, 0, 32, 32);
                render_tile(&mut canvas, &texture, sprite_rect, screen_rect);
            }

            if !cell.hidden {
                let sprite_rect = Rect::new(32, 0, 32, 32);
                render_tile(&mut canvas, &texture, sprite_rect, screen_rect);

                if cell.is_bomb {
                    let sprite_rect = Rect::new(32 * (position.0 + 2), 0, 32, 32);
                    render_tile(&mut canvas, &texture, sprite_rect, screen_rect);
                }
            }
        }

        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn render_tile(canvas: &mut WindowCanvas, texture: &Texture, sprite_rect: Rect, screen_rect: Rect) {
    canvas.copy(&texture, sprite_rect, screen_rect).unwrap();
}

fn handle_left_click(x: i32, y: i32, cell_map: &mut HashMap<(i32, i32), Cell>) {
    let key = ((x - 6)  / 32, (y - 50) / 32);
    match cell_map.get_mut(&key) {
        Some(cell) => cell.reveal(),
        _ => {}
    }
}

struct Cell {
    hidden: bool,
    is_bomb: bool,
    adjacent_bombs: u8
}

impl Cell {
    pub fn reveal (&mut self) {
        self.hidden = false;
    }
}