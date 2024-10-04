use rand::{prelude::SliceRandom, thread_rng};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Paragraph},
};

mod card;
use card::{Card, Rank, Suit};

#[derive(PartialEq)]
enum Turn {
    My,
    Bust,
    Check,
    NewGo,
    NewDeck,
}

#[derive(Default)]
struct State {
    should_quit: bool,
    user_input: String,
    submit_input: bool,
    valid_input: bool,
    my_hand: Vec<Card>,
    dealer_hand: Vec<Card>,
    turn: Turn,
    deck: Vec<Card>,
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut state = State::default();

    while !state.should_quit {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Backspace => state.input_backspace(),
                    KeyCode::Char(char) => state.input_char(char),
                    KeyCode::Enter => state.input_enter(),
                    _ => (),
                }
            }
        }

        state.update();

        terminal.draw(|frame| {
            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(8),
                    Constraint::Percentage(100),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(frame.area());

            let table = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_layout[1]);

            let title = vec![
                "██████╗ ██╗      █████╗  ██████╗██╗  ██╗     ██╗ █████╗  ██████╗██╗  ██╗".into(),
                "██╔══██╗██║     ██╔══██╗██╔════╝██║ ██╔╝     ██║██╔══██╗██╔════╝██║ ██╔╝".into(),
                "██████╔╝██║     ███████║██║     █████╔╝      ██║███████║██║     █████╔╝ ".into(),
                "██╔══██╗██║     ██╔══██║██║     ██╔═██╗ ██   ██║██╔══██║██║     ██╔═██╗ ".into(),
                "██████╔╝███████╗██║  ██║╚██████╗██║  ██╗╚█████╔╝██║  ██║╚██████╗██║  ██╗".into(),
                "╚═════╝ ╚══════╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝".into(),
            ];

            let mut title_paragraph = Paragraph::new(title)
                .alignment(Alignment::Center)
                .block(Block::bordered());

            if state.turn == Turn::Bust {
                title_paragraph = title_paragraph.red();
            } else if state.turn == Turn::Check {
                title_paragraph = title_paragraph.light_blue();
            }

            frame.render_widget(title_paragraph, main_layout[0]);

            frame.render_widget(
                Paragraph::new(state.my_hand()).block(Block::bordered().title("My Hand")),
                table[0],
            );
            frame.render_widget(
                Paragraph::new(state.dealer_hand()).block(Block::bordered().title("Dealer Hand")),
                table[1],
            );

            frame.render_widget(
                Paragraph::new("Cards remaining: ".to_string() + &state.deck.len().to_string())
                    .block(Block::bordered().title("Deck")),
                main_layout[2],
            );

            frame.render_widget(
                Paragraph::new(state.user_input.clone())
                    .style(Style::new().bold())
                    .block(
                        Block::bordered()
                            .title("User Input")
                            .title_alignment(Alignment::Center),
                    )
                    .alignment(Alignment::Center),
                main_layout[3],
            );
        })?;
    }

    ratatui::restore();
    Ok(())
}

impl Default for Turn {
    fn default() -> Self {
        Self::NewDeck
    }
}

impl State {
    pub fn input_backspace(&mut self) {
        self.user_input.pop();
    }

    pub fn input_char(&mut self, char: char) {
        self.user_input.push(char);
    }

    pub fn input_enter(&mut self) {
        self.submit_input = true;
    }

    pub fn update(&mut self) {
        if self.valid_input {
            self.valid_input = false;
        }

        if !self.submit_input {
            return;
        }

        match self.user_input.as_str() {
            "quit" => self.should_quit = true,
            "hit" => {
                self.valid_input = true;
                self.my_hand.push(self.deck.pop().unwrap());
            }
            "stay" => {
                self.valid_input = true;
                self.turn = Turn::Check;
            }
            "new" => {
                self.valid_input = true;
                self.turn = Turn::NewDeck;
            }
            _ => {
                if self.turn == Turn::Bust || self.turn == Turn::Check {
                    self.valid_input = true;
                    self.turn = Turn::NewGo;
                }
            }
        }

        self.user_input.clear();
        self.submit_input = false;

        if !self.valid_input {
            return;
        }

        match self.turn {
            Turn::NewDeck => {
                self.generate_deck();

                self.my_hand.clear();
                self.dealer_hand.clear();
                self.my_hand.push(self.deck.pop().unwrap());
                self.dealer_hand.push(self.deck.pop().unwrap());
                self.my_hand.push(self.deck.pop().unwrap());
                self.dealer_hand.push(self.deck.pop().unwrap());

                self.turn = Turn::My;
            }
            Turn::NewGo => {
                self.my_hand.clear();
                self.dealer_hand.clear();
                self.my_hand.push(self.deck.pop().unwrap());
                self.dealer_hand.push(self.deck.pop().unwrap());
                self.my_hand.push(self.deck.pop().unwrap());
                self.dealer_hand.push(self.deck.pop().unwrap());

                self.turn = Turn::My;
            }
            Turn::My => {
                let mut total = 0;
                for hand in &self.my_hand {
                    total += hand.value();
                }

                if total > 21 {
                    self.turn = Turn::Bust;
                }
            }
            Turn::Check => {
                let mut total = 0;
                for hand in &self.dealer_hand {
                    total += hand.value();
                }

                while total <= 16 {
                    let card = self.deck.pop().unwrap();
                    total += card.value();
                    self.dealer_hand.push(card);
                }
            }
            _ => (),
        }
    }

    pub fn my_hand(&self) -> String {
        let mut output = String::new();

        for card in &self.my_hand {
            output += "| ";
            output += card.value().to_string().as_str();
            output += " ";
        }

        output + "|"
    }

    pub fn dealer_hand(&self) -> String {
        let mut output = String::new();

        for card in &self.dealer_hand {
            output += "| ";
            output += card.value().to_string().as_str();
            output += " ";
        }

        output + "|"
    }

    fn generate_deck(&mut self) {
        self.deck.clear();

        let suits = [Suit::Diamond, Suit::Heart, Suit::Club, Suit::Spade];
        let ranks = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];

        for suit in &suits {
            for rank in &ranks {
                self.deck.push(Card {
                    suit: suit.clone(),
                    rank: rank.clone(),
                });
            }
        }

        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }
}
