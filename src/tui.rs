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

use crate::card::Card;
use crate::game::{Game, State, Winner, PLAYING_DECK_SIZE};

pub struct TUI<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    paragraph_title: Paragraph<'a>,
    paragraph_title_compact: Paragraph<'a>,
    main_layout: Layout,
    main_layout_compact: Layout,
    table: Layout,
    table_compact: Layout,
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

        Self {
            terminal,
            paragraph_title,
            paragraph_title_compact: Paragraph::new("Blackjack").alignment(Alignment::Center),
            main_layout,
            main_layout_compact,
            table,
            table_compact,
        }
    }

    pub fn draw(&mut self, game: &Game) -> std::io::Result<()> {
        self.terminal.draw(|frame| {
            let (my_hand_widget, dealer_hand_widget) = Self::create_hand_widgets(game);

            let area = frame.area();

            if area.width < 26 || area.height < 12 {
                frame.render_widget("TOO SMALL!", area);
                return;
            }

            match game.compact_mode() {
                true => {
                    let main_layout = self.main_layout_compact.split(area);

                    frame.render_widget(&self.paragraph_title_compact, main_layout[0]);

                    let table = self.table_compact.split(main_layout[1]);

                    frame.render_widget(my_hand_widget, table[0]);
                    frame.render_widget(dealer_hand_widget, table[1]);

                    let text = format!(
                        "Profit: {}, Bet: {}\nCards remaining: {}",
                        game.profit(),
                        game.bet(),
                        game.deck_length()
                    );

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
                    let main_layout = self.main_layout.split(area);

                    frame.render_widget(&self.paragraph_title, main_layout[0]);

                    let table = self.table.split(main_layout[1]);

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

    fn hand_as_string(hand: &Vec<Card>, compact_mode: bool) -> String {
        let mut output = String::new();

        match compact_mode {
            true => {
                for card in hand {
                    output += &format!("{}\n", card.as_compact_string());
                }
            }
            false => {
                let mut is_first_card = true;
                let mut first_card = Vec::new();
                let mut second_card;

                for card in hand {
                    if is_first_card {
                        first_card = card.as_art_string_lines();
                        is_first_card = !is_first_card;
                    } else {
                        second_card = card.as_art_string_lines();
                        is_first_card = !is_first_card;

                        for i in 0..first_card.len() {
                            output += &format!("{} {}\n", first_card[i], second_card[i]);
                        }
                    }
                }

                if !is_first_card {
                    for line in first_card {
                        output += &format!("{}\n", line);
                    }
                }
            }
        }

        output
    }

    fn create_hand_widgets(game: &Game) -> (Paragraph, Paragraph) {
        let paragraph_me =
            Paragraph::new(Self::hand_as_string(game.my_hand(), game.compact_mode()))
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .title("My Hand")
                        .title_alignment(Alignment::Center),
                );

        let paragraph_dealer = Paragraph::new(Self::hand_as_string(
            game.dealer_hand(),
            game.compact_mode(),
        ))
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .title("Dealer Hand")
                .title_alignment(Alignment::Center),
        );

        match game.state() {
            State::Result(Winner::None) => {
                (paragraph_me.light_blue(), paragraph_dealer.light_blue())
            }
            State::Result(Winner::Me) => (paragraph_me.light_green(), paragraph_dealer.light_red()),
            State::Result(Winner::Dealer) => {
                (paragraph_me.light_red(), paragraph_dealer.light_green())
            }
            _ => (paragraph_me, paragraph_dealer),
        }
    }

    fn create_profit_widget(profit: f32, bet: f32, layout_area: Rect) -> Paragraph<'a> {
        let mut paragraph_profit = String::new();

        let mut centre_line = format!("Profit: {}", profit);

        let table_height = (layout_area.height - 2) as i32;
        let half_table_height = table_height / 2;

        let mut double_line = false;

        if table_height % 2 == 0 {
            centre_line += &format!("\n    Bet: {}", bet);
            double_line = true;
        } else {
            centre_line += &format!(", Bet: {}", bet);
        }

        let lines = (profit.abs() / 100.0) as i32;

        let bar = "█".repeat((layout_area.width - 4) as usize);

        let mut skip_next = false;

        for height in 0..table_height {
            if skip_next {
                skip_next = !skip_next;
                continue;
            }

            if double_line {
                if height == half_table_height - 1 {
                    paragraph_profit += &centre_line;
                    skip_next = !skip_next;
                    paragraph_profit += "\n";
                    continue;
                }

                if profit > 0.0 {
                    if height > half_table_height - 2 - lines && height < half_table_height - 1 {
                        paragraph_profit += &bar;
                    }
                } else {
                    if height < half_table_height + 1 + lines && height > half_table_height {
                        paragraph_profit += &bar;
                    }
                }
            } else {
                if height == half_table_height {
                    paragraph_profit += &centre_line;
                    paragraph_profit += "\n";
                    continue;
                }

                if profit > 0.0 {
                    if height > half_table_height - 1 - lines && height < half_table_height {
                        paragraph_profit += &bar;
                    }
                } else {
                    if height < half_table_height + 1 + lines && height > half_table_height {
                        paragraph_profit += &bar;
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

        match profit.total_cmp(&0.0) {
            Ordering::Equal => create_paragraph_profit,
            Ordering::Greater => create_paragraph_profit.light_green(),
            Ordering::Less => create_paragraph_profit.light_red(),
        }
    }

    fn create_deck_widget(cards_remaining: usize, layout_area: Rect) -> Paragraph<'a> {
        let mut paragraph_deck = format!("Cards remaining: {}\n\n", cards_remaining);

        let deck_height = layout_area.height - 4;
        let cutoff = 1.0 - (cards_remaining as f32 / PLAYING_DECK_SIZE as f32);

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
