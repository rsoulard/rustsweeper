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

pub struct Cell {
    hidden: bool,
    is_trap: bool,
    is_flagged: bool,
    adjacent_trap_count: u8
}

impl Cell {
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    fn reveal (&mut self) {
        self.hidden = false;
    }

    pub fn is_trap(&self) -> bool {
        self.is_trap
    }

    fn make_trap(&mut self) {
        self.is_trap = true;
    }

    pub fn is_flagged(&self) -> bool {
        self.is_flagged
    }

    fn toggle_flag(&mut self) {
        self.is_flagged = !self.is_flagged;
    }

    pub fn get_adjacent_trap_count(&self) -> u8{
        self.adjacent_trap_count
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
            is_flagged: false,
            adjacent_trap_count: 0 
        }
    }
}

pub struct CellMap {
    width : usize,
    height : usize,
    cells : Vec::<Cell>,
    has_generated: bool,
    trap_count: usize
}

impl CellMap {
    pub fn new(width: usize, height: usize, trap_count: usize) -> Self {
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

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_cell(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
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

    pub fn reveal_cell(&mut self, x: i32, y: i32) {
        if !self.has_generated {
            self.generate(x, y);
        }

        if let Some(cell) = self.get_cell(x, y) {
            match cell {
                Cell {is_trap: false, is_flagged: false, adjacent_trap_count: 0, hidden: true} => {
                    cell.reveal();
                    for offset in ADJACENCY_OFFSET_TABLE {
                        self.reveal_cell(x + offset.0, y + offset.1);
                    }
                },
                Cell {is_flagged: false, ..} => {
                    cell.reveal();
                }
                _ => {}
            }
        }
    }

    pub fn flag_cell(&mut self, x: i32, y: i32) {
        if let Some(cell) = self.get_cell(x, y) {
            cell.toggle_flag();
        }
    }

    pub fn generate(&mut self, first_x: i32, first_y: i32) {
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