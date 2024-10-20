use std::cmp::Ordering;

use ratatui::{
    layout::{Alignment, Rect},
    style::Stylize,
    widgets::{Block, Paragraph},
};

pub fn create_profit_widget<'a>(profit: f32, bet: f32, layout_area: Rect) -> Paragraph<'a> {
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
