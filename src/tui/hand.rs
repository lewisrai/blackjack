use ratatui::{
    layout::Alignment,
    style::Stylize,
    widgets::{block::Title, Block, Paragraph},
};

use super::{Card, DrawMode, Game, State, Winner};

pub fn create_hand_widgets(game: &Game, draw_mode: DrawMode) -> (Paragraph, Paragraph) {
    let title1 = Title::from("My Hand").alignment(Alignment::Center);
    let title2 = Title::from(match draw_mode {
        DrawMode::Warn => "",
        DrawMode::Compact => match game.state() {
            State::MyTurn => "q|h|s",
            State::Result(_) => "q|n|i|d",
        },
        DrawMode::NormalCompact | DrawMode::Normal => match game.state() {
            State::MyTurn => "(Q)uit, (H)it, (S)tay",
            State::Result(_) => "(Q)uit, (N)ew, (i/d): ↑/↓ Bet",
        },
    })
    .alignment(Alignment::Center)
    .position(ratatui::widgets::block::Position::Bottom);

    let paragraph_me = Paragraph::new(hand_as_string(game.my_hand(), draw_mode))
        .alignment(Alignment::Center)
        .block(Block::bordered().title(title1).title(title2));

    let paragraph_dealer = Paragraph::new(hand_as_string(game.dealer_hand(), draw_mode))
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .title("Dealer Hand")
                .title_alignment(Alignment::Center),
        );

    match game.state() {
        State::Result(Winner::None) => (paragraph_me.light_blue(), paragraph_dealer.light_blue()),
        State::Result(Winner::Me) => (paragraph_me.light_green(), paragraph_dealer.light_red()),
        State::Result(Winner::Dealer) => (paragraph_me.light_red(), paragraph_dealer.light_green()),
        _ => (paragraph_me, paragraph_dealer),
    }
}

fn hand_as_string(hand: &Vec<Card>, draw_mode: DrawMode) -> String {
    let mut output = String::new();

    match draw_mode {
        DrawMode::Warn => (),
        DrawMode::Compact => {
            for card in hand {
                output += &format!("{}\n", card.as_compact_string());
            }
        }
        DrawMode::NormalCompact => {
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
                        output += &format!("{}   {}\n", first_card[i], second_card[i]);
                    }
                }
            }

            if !is_first_card {
                for line in first_card {
                    output += &format!("{}\n", line);
                }
            }
        }
        DrawMode::Normal => {
            for card in hand {
                let text = card.as_art_string_lines();

                for line in text {
                    output += &format!("{}\n", line);
                }
            }
        }
    }

    output
}
