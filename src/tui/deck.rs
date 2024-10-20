use ratatui::{
    layout::Alignment,
    prelude::Rect,
    widgets::{Block, Paragraph},
};

use super::PLAYING_DECK_SIZE;

pub fn create_deck_widget<'a>(cards_remaining: usize, layout_area: Rect) -> Paragraph<'a> {
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
