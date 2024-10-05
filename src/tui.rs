use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
    Terminal,
};

use crate::game::Game;

pub struct TUI<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    paragraph_title: Paragraph<'a>,
}

impl<'a> TUI<'a> {
    pub fn new() -> Self {
        let terminal = ratatui::init();

        let title = vec![
            "██████╗ ██╗      █████╗  ██████╗██╗  ██╗     ██╗ █████╗  ██████╗██╗  ██╗".into(),
            "██╔══██╗██║     ██╔══██╗██╔════╝██║ ██╔╝     ██║██╔══██╗██╔════╝██║ ██╔╝".into(),
            "██████╔╝██║     ███████║██║     █████╔╝      ██║███████║██║     █████╔╝ ".into(),
            "██╔══██╗██║     ██╔══██║██║     ██╔═██╗ ██   ██║██╔══██║██║     ██╔═██╗ ".into(),
            "██████╔╝███████╗██║  ██║╚██████╗██║  ██╗╚█████╔╝██║  ██║╚██████╗██║  ██╗".into(),
            "╚═════╝ ╚══════╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝".into(),
        ];

        let paragraph_title = Paragraph::new(title)
            .alignment(Alignment::Center)
            .block(Block::bordered());

        Self {
            terminal,
            paragraph_title,
        }
    }

    pub fn draw(&mut self, game: &Game) -> std::io::Result<()> {
        self.terminal.draw(|frame| {
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(8), Constraint::Percentage(100)])
                .split(frame.area());

            let table = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(15),
                    Constraint::Percentage(35),
                    Constraint::Percentage(35),
                    Constraint::Percentage(15),
                ])
                .split(main_layout[1]);

            frame.render_widget(&self.paragraph_title, main_layout[0]);

            frame.render_widget(
                Paragraph::new("Too much...").block(
                    Block::bordered()
                        .title("Money")
                        .title_alignment(Alignment::Center),
                ),
                table[0],
            );

            frame.render_widget(
                Paragraph::new(game.my_hand()).block(
                    Block::bordered()
                        .title("My Hand")
                        .title_alignment(Alignment::Center),
                ),
                table[1],
            );

            frame.render_widget(
                Paragraph::new(game.dealer_hand()).block(
                    Block::bordered()
                        .title("Dealer Hand")
                        .title_alignment(Alignment::Center),
                ),
                table[2],
            );

            frame.render_widget(
                Paragraph::new("Cards remaining: ".to_string() + &game.deck_length()).block(
                    Block::bordered()
                        .title("Deck")
                        .title_alignment(Alignment::Center),
                ),
                table[3],
            );
        })?;

        Ok(())
    }
}
