use crate::board::Board;
use crate::keyboard::Keyboard;

pub struct Game {
    board: Board,
    score: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            score: 0,
        }
    }

    fn clear_screen(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn update(&mut self) {
        self.clear_screen();

        println!("Score: {}", self.score);
        println!("\nUse the arrow keys to move the tiles. Press 'q' to quit.\n");

        self.print_board();
    }

    pub fn init(&mut self) {
        self.board.add_random_cell();
        self.board.add_random_cell();

        self.print_board();
    }

    pub fn play(&mut self) {
        let mut keyboard = Keyboard::new(self);

        keyboard.start();
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn move_up(&mut self) {
        let score = self.board.move_up();

        self.score += score;

        self.update();

        if self.board.is_game_over() {
            println!("Game Over!");
            std::process::exit(0);
        }
    }

    pub fn move_down(&mut self) {
        let score = self.board.move_down();

        self.score += score;

        self.update();

        if self.board.is_game_over() {
            println!("Game Over!");
            std::process::exit(0);
        }
    }

    pub fn move_left(&mut self) {
        let score = self.board.move_left();

        self.score += score;

        self.update();

        if self.board.is_game_over() {
            println!("Game Over!");
            std::process::exit(0);
        }
    }

    pub fn move_right(&mut self) {
        let score = self.board.move_right();

        self.score += score;

        self.update();

        if self.board.is_game_over() {
            println!("Game Over!");
            std::process::exit(0);
        }
    }

    pub fn print_board(&self) {
        let board = self.get_board();

        println!("Score: {}", self.score);
        println!("\n Use the arrow keys to move the tiles. Press 'q' to quit.\n");
        board.print();
    }
}
