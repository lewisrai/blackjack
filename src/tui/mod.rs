use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Paragraph},
    Terminal,
};

use crate::card::Card;
use crate::game::{Game, State, Winner, PLAYING_DECK_SIZE};

mod deck;
use deck::create_deck_widget;
mod hand;
use hand::create_hand_widgets;
mod profit;
use profit::create_profit_widget;
mod stats;
use stats::create_stats_widget;

#[derive(Clone, Copy)]
enum DrawMode {
    Warn,
    Compact,
    NormalCompact,
    Normal,
}

pub struct TUI<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    paragraph_title: Paragraph<'a>,
    paragraph_title_compact: Paragraph<'a>,
    main_layout: Layout,
    main_layout_compact: Layout,
    table: Layout,
    table_compact: Layout,
    warning: Paragraph<'a>,
    draw_mode: DrawMode,
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

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(8), Constraint::Percentage(100)]);

        let main_layout_compact = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Percentage(100),
                Constraint::Length(4),
            ]);

        let table = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(20),
            ]);

        let table_compact = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)]);

        let warning = Paragraph::new("Blackjack")
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .title("Blackjack")
                    .title_alignment(Alignment::Center),
            );

        Self {
            terminal,
            paragraph_title,
            paragraph_title_compact: Paragraph::new("Blackjack").alignment(Alignment::Center),
            main_layout,
            main_layout_compact,
            table,
            table_compact,
            warning,
            draw_mode: DrawMode::Normal,
        }
    }

    pub fn draw(&mut self, game: &Game) -> std::io::Result<()> {
        self.terminal.draw(|frame| {
            let area = frame.area();

            if area.width < 26 || area.height < 12 {
                self.draw_mode = DrawMode::Warn;
            } else if area.width < 130 || area.height < 31 {
                self.draw_mode = DrawMode::Compact;
            } else if area.height < 45 {
                self.draw_mode = DrawMode::NormalCompact;
            } else {
                self.draw_mode = DrawMode::Normal;
            }

            let (my_hand_widget, dealer_hand_widget) = create_hand_widgets(game, self.draw_mode);

            match self.draw_mode {
                DrawMode::Warn => {
                    frame.render_widget(&self.warning, area);
                }
                DrawMode::Compact => {
                    let main_layout = self.main_layout_compact.split(area);

                    frame.render_widget(&self.paragraph_title_compact, main_layout[0]);

                    let table = self.table_compact.split(main_layout[1]);

                    frame.render_widget(my_hand_widget, table[0]);
                    frame.render_widget(dealer_hand_widget, table[1]);

                    let stats_widget =
                        create_stats_widget(game.profit(), game.bet(), game.deck_length());
                    frame.render_widget(stats_widget, main_layout[2]);
                }
                DrawMode::NormalCompact | DrawMode::Normal => {
                    let main_layout = self.main_layout.split(area);

                    frame.render_widget(&self.paragraph_title, main_layout[0]);

                    let table = self.table.split(main_layout[1]);

                    frame.render_widget(my_hand_widget, table[1]);
                    frame.render_widget(dealer_hand_widget, table[2]);

                    let profit_widget = create_profit_widget(game.profit(), game.bet(), table[0]);
                    frame.render_widget(profit_widget, table[0]);

                    let deck_widget = create_deck_widget(game.deck_length(), table[3]);
                    frame.render_widget(deck_widget, table[3]);
                }
            };
        })?;

        Ok(())
    }
}
