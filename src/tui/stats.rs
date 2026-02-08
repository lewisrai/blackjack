use ratatui::{
    layout::Alignment,
    widgets::{Block, Paragraph},
};

pub fn create_stats_widget<'a>(profit: f32, bet: f32, deck_length: usize) -> Paragraph<'a> {
    let text = format!(
        "Profit: {}, Bet: {}\nCards remaining: {}",
        profit, bet, deck_length
    );

    Paragraph::new(text).alignment(Alignment::Center).block(
        Block::bordered()
            .title("Stats")
            .title_alignment(Alignment::Center),
    )
}
