extern crate sdl2;

use sdl2::{pixels::Color, keyboard::Keycode, event::Event, image::LoadTexture, rect::Rect, render::{WindowCanvas, Texture}};
use std::{time::Duration, thread};
use rand::{self, thread_rng, Rng};

static ADJACENCY_OFFSET_TABLE: [(i32, i32); 8]  = [
    (0, 1), 
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1)
];

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

    let mut cell_map = CellMap::new(9, 9, 12);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | 
                Event:: KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event:: MouseButtonUp { x, y, ..} => {
                    handle_left_click(&mut cell_map, x, y)
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for x in 0..cell_map.width {
            for y in 0..cell_map.height {
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

    match cell {
        Cell {hidden: true, ..} => {
            let sprite_rect = Rect::new(0, 0, 32, 32);
            render(canvas, texture, sprite_rect, screen_rect);
        },
        Cell {hidden: false, ..} => {
            let sprite_rect = Rect::new(32, 0, 32, 32);
            render(canvas, texture, sprite_rect, screen_rect);

            match cell {
                Cell {is_trap: true, ..} => {
                    let sprite_rect = Rect::new(32 * 2, 0, 32, 32);
                    render(canvas, texture, sprite_rect, screen_rect);
                },
                Cell {is_trap: false, ..} if cell.adjacent_trap_count > 0 => {
                    let sprite_rect = Rect::new(32 * (2 + cell.adjacent_trap_count as i32), 0, 32, 32);
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

struct Cell {
    hidden: bool,
    is_trap: bool,
    adjacent_trap_count: u8
}

impl Cell {
    fn reveal (&mut self) {
        self.hidden = false;
    }

    fn make_trap(&mut self) {
        self.is_trap = true;
    }

    fn increment_adjacent_trap_count(&mut self) {
        self.adjacent_trap_count += 1;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self { 
            hidden: true, 
            is_trap: false, 
            adjacent_trap_count: 0 
        }
    }
}

struct CellMap {
    width : usize,
    height : usize,
    cells : Vec::<Cell>,
    has_generated: bool,
    trap_count: usize
}

impl CellMap {
    fn new(width: usize, height: usize, trap_count: usize) -> Self {
        let mut cells = Vec::<Cell>::with_capacity(width * height);

        let total = width * height;

        for _ in 0..total {
            cells.push(Cell { ..Default::default() });
        }

        Self { 
            cells,
            width,
            height,
            has_generated: false,
            trap_count
        }
    }

    fn get_cell(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        let x = x as usize;
        let y = y as usize;

        match x < self.width &&  y < self.height {
            true => Some(&mut self.cells[x + y * self.width]),
            false => None
        }
    }

    fn place_bomb(&mut self, x: i32, y: i32) {
        if let Some(cell) = self.get_cell(x, y) {
            cell.make_trap();
        }

        for offset in ADJACENCY_OFFSET_TABLE {
            if let Some(cell) = self.get_cell(x + offset.0, y + offset.1) {
                cell.increment_adjacent_trap_count();
            }
        }
    }

    fn reveal_cell(&mut self, x: i32, y: i32) {
        if !self.has_generated {
            self.generate(x, y);
        }

        if let Some(cell) = self.get_cell(x, y) {
            match cell {
                Cell {is_trap: false, adjacent_trap_count: 0, hidden: true} => {
                    cell.reveal();
                    for offset in ADJACENCY_OFFSET_TABLE {
                        self.reveal_cell(x + offset.0, y + offset.1);
                    }
                },
                _ => {
                    cell.reveal();
                }
                //_ => {}
            }
        }
    }

    fn generate(&mut self, first_x: i32, first_y: i32) {
        let mut traps_placed = 0;
        let maximum_index = self.width * self.height;

        while traps_placed < self.trap_count {
            let random_index = thread_rng().gen_range(0..maximum_index);
            let (x, y) = ((random_index % self.width) as i32, (random_index / self.height) as i32);

            if (x, y) == (first_x, first_y) {
                continue;
            }

            if let Some(Cell {is_trap: false, ..}) = self.get_cell(x, y) {
                self.place_bomb(x, y);
                traps_placed += 1;
            }
        }
        self.has_generated = true;
    }
}