use std::cmp::Ordering;
use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Block, Paragraph},
    Terminal,
};

use crate::game::{Game, State, Winner};

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

            let mut paragraph_me = Paragraph::new(game.my_hand())
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .title("My Hand")
                        .title_alignment(Alignment::Center),
                );

            let mut paragraph_dealer = Paragraph::new(game.dealer_hand())
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .title("Dealer Hand")
                        .title_alignment(Alignment::Center),
                );

            if game.state() == State::Result {
                match game.winner() {
                    Winner::None => {
                        paragraph_me = paragraph_me.light_blue();
                        paragraph_dealer = paragraph_dealer.light_blue();
                    }
                    Winner::Me => {
                        paragraph_me = paragraph_me.light_green();
                        paragraph_dealer = paragraph_dealer.light_red();
                    }
                    Winner::Dealer => {
                        paragraph_me = paragraph_me.light_red();
                        paragraph_dealer = paragraph_dealer.light_green();
                    }
                }
            }

            frame.render_widget(&self.paragraph_title, main_layout[0]);

            let mut paragraph_profit = String::new();
            let profit = game.profit();

            let table_0 = table[0];

            let mut centre_line = "Profit : ".to_string() + &profit.to_string();

            let table_height = table_0.height - 2;
            let mut double_line = false;

            if table_height % 2 == 0 {
                centre_line += &"\n    Bet: ".to_string();
                centre_line += &game.bet().to_string();
                double_line = true;
            } else {
                centre_line += &", Bet: ".to_string();
                centre_line += &game.bet().to_string();
            }

            let mut skip_next = false;

            let lines = (profit.abs() / 100) as i32;

            for height in 0..table_height {
                if skip_next {
                    skip_next = false;
                    continue;
                }

                for _ in 0..table_0.width - 4 {
                    if double_line {
                        if height == table_height / 2 - 1 {
                            paragraph_profit += &centre_line;
                            skip_next = true;
                            break;
                        }

                        if profit > 0 {
                            if height > table_height / 2 - 2 - lines as u16
                                && height < table_height / 2 - 1
                            {
                                paragraph_profit += "█";
                            }
                        } else {
                            if height < table_height / 2 + 1 + lines as u16
                                && height > table_height / 2
                            {
                                paragraph_profit += "█";
                            }
                        }
                    } else {
                        if height == table_height / 2 {
                            paragraph_profit += &centre_line;
                            break;
                        }

                        if profit > 0 {
                            if height > table_height / 2 - 1 - lines as u16
                                && height < table_height / 2
                            {
                                paragraph_profit += "█";
                            }
                        } else {
                            if height < table_height / 2 + 1 + lines as u16
                                && height > table_height / 2
                            {
                                paragraph_profit += "█";
                            }
                        }
                    }
                }

                paragraph_profit += "\n";
            }

            let mut create_paragraph_profit = Paragraph::new(paragraph_profit)
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .title("Money")
                        .title_alignment(Alignment::Center),
                );

            create_paragraph_profit = match profit.cmp(&0) {
                Ordering::Equal => create_paragraph_profit,
                Ordering::Greater => create_paragraph_profit.light_green(),
                Ordering::Less => create_paragraph_profit.light_red(),
            };

            frame.render_widget(create_paragraph_profit, table_0);

            frame.render_widget(paragraph_me, table[1]);

            frame.render_widget(paragraph_dealer, table[2]);

            let table_3 = table[3];
            let cards_remaining = game.deck_length();

            let mut paragraph_deck =
                "Cards remaining: ".to_string() + &cards_remaining.to_string() + "\n\n";

            let deck_height = table_3.height - 4;
            let cutoff = 1.0 - (cards_remaining as f32 / 104.0);

            for height in 0..deck_height {
                if height as f32 / deck_height as f32 > cutoff {
                    for _ in 0..table_3.width - 4 {
                        paragraph_deck += "█";
                    }
                }

                paragraph_deck += "\n";
            }

            frame.render_widget(
                Paragraph::new(paragraph_deck)
                    .alignment(Alignment::Center)
                    .block(
                        Block::bordered()
                            .title("Deck")
                            .title_alignment(Alignment::Center),
                    ),
                table_3,
            );
        })?;

        Ok(())
    }
}
