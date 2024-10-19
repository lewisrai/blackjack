use std::cmp::Ordering;
use std::io::Stdout;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::Rect,
    style::Stylize,
    widgets::{Block, Paragraph},
    Terminal,
};

use crate::game::{Game, State, Winner};

pub struct TUI<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    paragraph_title: Paragraph<'a>,
    paragraph_title_compact: Paragraph<'a>,
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
            paragraph_title_compact: Paragraph::new("Blackjack").alignment(Alignment::Center),
        }
    }

    pub fn draw(&mut self, game: &Game) -> std::io::Result<()> {
        self.terminal.draw(|frame| {
            match game.compact_mode() {
                true => {
                    let main_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(vec![
                            Constraint::Length(1),
                            Constraint::Percentage(100),
                            Constraint::Length(4),
                        ])
                        .split(frame.area());

                    frame.render_widget(&self.paragraph_title_compact, main_layout[0]);

                    let table = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(main_layout[1]);

                    let (my_hand_widget, dealer_hand_widget) = Self::create_hand_widgets(game);

                    frame.render_widget(my_hand_widget, table[0]);
                    frame.render_widget(dealer_hand_widget, table[1]);

                    let mut text = "Profit: ".to_string() + &game.profit().to_string();
                    text += ", Bet: ";
                    text += &game.bet().to_string();
                    text += "\nCards remaining: ";
                    text += &game.deck_length().to_string();

                    frame.render_widget(
                        Paragraph::new(text).alignment(Alignment::Center).block(
                            Block::bordered()
                                .title("Stats")
                                .title_alignment(Alignment::Center),
                        ),
                        main_layout[2],
                    );
                }
                false => {
                    let main_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(vec![Constraint::Length(8), Constraint::Percentage(100)])
                        .split(frame.area());

                    frame.render_widget(&self.paragraph_title, main_layout[0]);

                    let table = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(vec![
                            Constraint::Percentage(20),
                            Constraint::Percentage(30),
                            Constraint::Percentage(30),
                            Constraint::Percentage(20),
                        ])
                        .split(main_layout[1]);

                    let (my_hand_widget, dealer_hand_widget) = Self::create_hand_widgets(game);

                    frame.render_widget(my_hand_widget, table[1]);
                    frame.render_widget(dealer_hand_widget, table[2]);

                    let profit_widget =
                        Self::create_profit_widget(game.profit(), game.bet(), table[0]);
                    frame.render_widget(profit_widget, table[0]);

                    let deck_widget = Self::create_deck_widget(game.deck_length(), table[3]);
                    frame.render_widget(deck_widget, table[3]);
                }
            };
        })?;

        Ok(())
    }

    fn create_hand_widgets(game: &Game) -> (Paragraph, Paragraph) {
        let paragraph_me = Paragraph::new(game.my_hand())
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .title("My Hand")
                    .title_alignment(Alignment::Center),
            );

        let paragraph_dealer = Paragraph::new(game.dealer_hand())
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .title("Dealer Hand")
                    .title_alignment(Alignment::Center),
            );

        if game.state() == State::Result {
            match game.winner() {
                Winner::None => (paragraph_me.light_blue(), paragraph_dealer.light_blue()),
                Winner::Me => (paragraph_me.light_green(), paragraph_dealer.light_red()),
                Winner::Dealer => (paragraph_me.light_red(), paragraph_dealer.light_green()),
            }
        } else {
            (paragraph_me, paragraph_dealer)
        }
    }

    fn create_profit_widget(profit: i32, bet: i32, layout_area: Rect) -> Paragraph<'a> {
        let mut paragraph_profit = String::new();

        let mut centre_line = "Profit : ".to_string() + &profit.to_string();

        let table_height = layout_area.height - 2;
        let mut double_line = false;

        if table_height % 2 == 0 {
            centre_line += &"\n    Bet: ".to_string();
            centre_line += &bet.to_string();
            double_line = true;
        } else {
            centre_line += &", Bet: ".to_string();
            centre_line += &bet.to_string();
        }

        let mut skip_next = false;

        let lines = (profit.abs() / 100) as i32;

        for height in 0..table_height {
            if skip_next {
                skip_next = false;
                continue;
            }

            for _ in 0..layout_area.width - 4 {
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
                        if height < table_height / 2 + 1 + lines as u16 && height > table_height / 2
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
                        if height > table_height / 2 - 1 - lines as u16 && height < table_height / 2
                        {
                            paragraph_profit += "█";
                        }
                    } else {
                        if height < table_height / 2 + 1 + lines as u16 && height > table_height / 2
                        {
                            paragraph_profit += "█";
                        }
                    }
                }
            }

            paragraph_profit += "\n";
        }

        let create_paragraph_profit = Paragraph::new(paragraph_profit)
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .title("Money")
                    .title_alignment(Alignment::Center),
            );

        match profit.cmp(&0) {
            Ordering::Equal => create_paragraph_profit,
            Ordering::Greater => create_paragraph_profit.light_green(),
            Ordering::Less => create_paragraph_profit.light_red(),
        }
    }

    fn create_deck_widget(cards_remaining: usize, layout_area: Rect) -> Paragraph<'a> {
        let mut paragraph_deck =
            "Cards remaining: ".to_string() + &cards_remaining.to_string() + "\n\n";

        let deck_height = layout_area.height - 4;
        let cutoff = 1.0 - (cards_remaining as f32 / 104.0);

        for height in 0..deck_height {
            if height as f32 / deck_height as f32 > cutoff {
                for _ in 0..layout_area.width - 4 {
                    paragraph_deck += "█";
                }
            }

            paragraph_deck += "\n";
        }

        Paragraph::new(paragraph_deck)
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .title("Deck")
                    .title_alignment(Alignment::Center),
            )
    }
}
