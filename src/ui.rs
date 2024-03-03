use crossterm::style::Stylize;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use tui_big_text::{BigText, PixelSize};

use crate::{app::App, icons::IconManger};

const LARGE_SIZE: &str = "\x1b[1m";

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Initialize icons
    let icon_manager = IconManger::new();

    let wether_block = Block::default()
        .title("Wether Forecast")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    // render the block
    frame.render_widget(&wether_block, frame.size());

    // get the inner area of the block
    let area = wether_block.inner(frame.size());

    // Create layout
    // let [search_layout, temp_layout, forecast_layout] = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints([
    //         Constraint::Percentage(10),
    //         Constraint::Percentage(20),
    //         Constraint::Fill(1),
    //     ])
    //     .split(area);
    let [search_layout, temp_layout, forecast_layout] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Fill(1),
    ])
    .areas(area);

    let mut wether_text;
    if let Some(wether) = &app.wether {
        let icon = icon_manager.get_icon(wether.current_conditions.icon.clone());
        wether_text = Paragraph::new(format!(
            "Wether forecast for {} is {} {}",
            &wether.address, &wether.current_conditions.temp, icon
        ))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .centered();
    } else {
        wether_text = Paragraph::new(format!("Press `r` to get wether forecast for anantapur",))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .centered();
    }

    wether_text = wether_text.block(wether_block);
    frame.render_widget(wether_text, frame.size());
}

// pub fn render_icon<'a>(icon: String) -> Option<BigText<'a>> {
//     let big_text = BigText::builder()
//         .pixel_size(PixelSize::Sextant)
//         .style(Style::default())
//         .lines(vec![icon.white().to_string().into()])
//         .build();
//     match big_text {
//         Ok(text) => Some(text),
//         Err(_) => None,
//     }
// }

// pub fn render_icon_widget<'a>(icon: String, frame: &mut Frame, area: Rect) {
// let wether_block = Block::default()
//     .title("Wether Forecast")
//     .title_alignment(Alignment::Center)
//     .borders(Borders::ALL)
//     .border_type(BorderType::Rounded);
// frame.render_widget(&wether_block, area);
// let area = wether_block.inner(area);
// let icon_text_widget = render_icon(icon).unwrap();
//     frame.render_widget(icon_text_widget, area);
// }
