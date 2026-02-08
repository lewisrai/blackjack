use ratatui::{
    layout::{Alignment, Rect},
    widgets::{Block, Paragraph},
};

use super::PLAYING_DECK_SIZE;

pub fn create_deck_widget<'a>(cards_remaining: usize, layout_area: Rect) -> Paragraph<'a> {
    let mut paragraph_deck = format!("Cards remaining: {}\n\n", cards_remaining);

    let deck_height = layout_area.height - 4;
    let cutoff = 1.0 - (cards_remaining as f32 / PLAYING_DECK_SIZE as f32);

    let bar = "█".repeat((layout_area.width - 4) as usize);

    for height in 0..deck_height {
        if height as f32 / deck_height as f32 > cutoff {
            paragraph_deck += &bar;
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
