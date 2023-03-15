extern crate sdl2;

mod cell_map;
use crate::cell_map::*;

use sdl2::{pixels::Color, keyboard::Keycode, event::Event, image::LoadTexture, rect::Rect, render::{WindowCanvas, Texture}, mouse::MouseButton};
use std::{time::Duration, thread};

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

    let mut cell_map = CellMap::new(9, 9, 10);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | 
                Event:: KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event:: MouseButtonUp { x, y, mouse_btn, ..} => {
                    match mouse_btn {
                        MouseButton::Left => {handle_left_click(&mut cell_map, x, y)},
                        MouseButton::Right => {handle_right_click(&mut cell_map, x, y)},
                        _ => {}
                    }

                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let (map_width, map_height) = cell_map.get_dimensions();
        
        for x in 0..map_width {
            for y in 0..map_height {
                if let Some(cell) = cell_map.get_cell(x as i32, y as i32) {
                    render_cell(&mut canvas, &texture, x as i32, y as i32, cell)
                }
            }
        }

        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn render_cell(canvas: &mut WindowCanvas, texture: &Texture, x: i32, y: i32, cell: &Cell) {
    let screen_rect = Rect::new(x * 32 + 6, y * 32 + 50, 32, 32);

    match cell.is_hidden() {
        true => {
            let sprite_rect = Rect::new(0, 0, 32, 32);
            render(canvas, texture, sprite_rect, screen_rect);
            if cell.is_flagged() {
                let sprite_rect = Rect::new(32 * 11, 0, 32, 32);
                render(canvas, texture, sprite_rect, screen_rect);
            }
        },
        false => {
            let sprite_rect = Rect::new(32, 0, 32, 32);
            render(canvas, texture, sprite_rect, screen_rect);

            match cell.is_trap() {
                true => {
                    let sprite_rect = Rect::new(32 * 2, 0, 32, 32);
                    render(canvas, texture, sprite_rect, screen_rect);
                },
                false if cell.get_adjacent_trap_count() > 0 => {
                    let sprite_rect = Rect::new(32 * (2 + cell.get_adjacent_trap_count() as i32), 0, 32, 32);
                    render(canvas, texture, sprite_rect, screen_rect);
                },
                _ => {}
            }
        }
    }  
}

fn render(canvas: &mut WindowCanvas, texture: &Texture, sprite_rect: Rect, screen_rect: Rect) {
    canvas.copy(texture, sprite_rect, screen_rect).unwrap();
}

fn handle_left_click(cell_map: &mut CellMap, x: i32, y: i32) {
    let (x, y) = ((x - 6)  / 32, (y - 50) / 32);

    cell_map.reveal_cell(x, y);
}

fn handle_right_click(cell_map: &mut CellMap, x: i32, y: i32) {
    let (x, y) = ((x - 6)  / 32, (y - 50) / 32);

    cell_map.flag_cell(x, y);
}