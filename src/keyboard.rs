use crate::game::Game;
use crossterm::event::{read, Event, KeyCode, KeyEvent};

pub struct Keyboard<'a> {
    game: &'a mut Game,
}

impl<'a> Keyboard<'a> {
    pub fn new(game: &mut Game) -> Keyboard {
        Keyboard { game }
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) {
        if event.kind != crossterm::event::KeyEventKind::Release {
            return;
        }

        match event.code {
            KeyCode::Char('q') => {
                // Quit the game
                std::process::exit(0);
            }
            KeyCode::Up => {
                self.game.move_up();
            }
            KeyCode::Down => {
                self.game.move_down();
            }
            KeyCode::Left => {
                self.game.move_left();
            }
            KeyCode::Right => {
                self.game.move_right();
            }
            _ => {}
        }
    }

    pub fn start(&mut self) {
        loop {
            if let Ok(Event::Key(event)) = read() {
                self.handle_key_event(event);
            }
        }
    }
}
