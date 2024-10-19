use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

mod card;
mod game;
use game::{Game, Input};
mod tui;
use tui::TUI;

fn main() -> std::io::Result<()> {
    let mut tui = TUI::new();
    let mut game = Game::new();

    loop {
        let mut input = Input::None;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => input = Input::Hit,
                    KeyCode::Char('s') => input = Input::Stay,
                    KeyCode::Char('n') => input = Input::New,
                    KeyCode::Char('c') => input = Input::CompactMode,
                    _ => (),
                }
            }
        }

        game.update(input);
        tui.draw(&game)?;
    }

    ratatui::restore();
    Ok(())
}
