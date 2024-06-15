use rand::Rng;

pub struct Board {
    grid: [[Option<u32>; 4]; 4],
}

impl Board {
    pub fn new() -> Board {
        // Create a new board with random cells
        Board {
            grid: [[None; 4]; 4],
        }
    }

    pub fn add_random_cell(&mut self) {
        // Add a random cell to the board
        let mut empty_cells = vec![];
        for y in 0..4 {
            for x in 0..4 {
                if self.grid[y][x].is_none() {
                    empty_cells.push((y, x));
                }
            }
        }
        if empty_cells.is_empty() {
            return;
        }
        let (y, x) = empty_cells[rand::thread_rng().gen_range(0..empty_cells.len())];
        self.grid[y][x] = Some(if rand::thread_rng().gen_range(0..10) == 0 {
            4
        } else {
            2
        });
    }

    pub fn print(&self) {
        for y in 0..4 {
            for x in 0..4 {
                if let Some(cell) = self.grid[y][x] {
                    // print!("{:4} ", cell);
                    print!("[{}]", cell);
                } else {
                    print!("[ ]");
                }
            }
            println!();
        }
    }

    pub fn slide_left(&mut self) -> u32 {
        let mut score = 0;
        for y in 0..4 {
            let mut row = vec![];
            for x in 0..4 {
                if let Some(cell) = self.grid[y][x] {
                    if let Some(last) = row.last_mut() {
                        if *last == cell {
                            *last *= 2;
                            score += *last;
                            continue;
                        }
                    }
                    row.push(cell);
                }
            }
            for x in 0..4 {
                self.grid[y][x] = row.get(x).cloned();
            }
        }

        self.add_new_number();

        score
    }

    pub fn slide_right(&mut self) -> u32 {
        let mut score = 0;
        for y in 0..4 {
            let mut row = vec![];
            for x in 0..4 {
                // get the cell from the right side of the grid
                if let Some(cell) = self.grid[y][3 - x] {
                    // if the cell is the same as the last cell in the row
                    if let Some(last) = row.last_mut() {
                        if *last == cell {
                            *last *= 2;
                            score += *last;
                            continue;
                        }
                    }
                    row.push(cell);
                }
            }
            // put the row back in the grid
            for x in 0..4 {
                self.grid[y][3 - x] = row.get(x).cloned();
            }
        }

        self.add_new_number();

        score
    }

    fn add_new_number(&mut self) {
        let mut empty_cells = vec![];
        for y in 0..4 {
            for x in 0..4 {
                if self.grid[y][x].is_none() {
                    empty_cells.push((y, x));
                }
            }
        }

        if empty_cells.is_empty() {
            return;
        }
        let (y, x) = empty_cells[rand::thread_rng().gen_range(0..empty_cells.len())];

        self.grid[y][x] = Some(if rand::thread_rng().gen_range(0..10) == 0 {
            4
        } else {
            2
        });
    }

    fn transpose(&mut self) {
        for y in 0..4 {
            for x in y..4 {
                let temp = self.grid[y][x];
                self.grid[y][x] = self.grid[x][y];
                self.grid[x][y] = temp;
            }
        }
    }

    pub fn move_up(&mut self) -> u32 {
        self.transpose();
        let score = self.slide_left();
        self.transpose();

        score
    }

    pub fn move_down(&mut self) -> u32 {
        self.transpose();
        let score = self.slide_right();
        self.transpose();

        score
    }

    pub fn move_left(&mut self) -> u32 {
        let score = self.slide_left();

        score
    }

    pub fn move_right(&mut self) -> u32 {
        let score = self.slide_right();

        score
    }

    pub fn is_game_over(&self) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.grid[y][x].is_none() {
                    return false;
                }
                if y > 0 && self.grid[y - 1][x] == self.grid[y][x] {
                    return false;
                }
                if x > 0 && self.grid[y][x - 1] == self.grid[y][x] {
                    return false;
                }
            }
        }
        true
    }
}
