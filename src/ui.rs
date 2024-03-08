use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, List, ListDirection, Paragraph},
    Frame,
};

use crate::{app::App, icons::IconManger};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Initialize icons
    let icon_manager = IconManger::new();

    let wether_block = Block::default()
        .title("Wether Forecast")
        .title_alignment(Alignment::Center)
        .borders(Borders::NONE)
        .style(Style::default().fg(Color::White).bg(Color::Black).bold())
        .border_type(BorderType::Rounded);

    // render the block
    frame.render_widget(&wether_block, frame.size());

    // get the inner area of the block
    let area = wether_block.inner(frame.size());

    // Create layout
    let [search_layout, temp_layout, forecast_layout] = Layout::vertical([
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Fill(1),
    ])
    .areas(area);

    // create search widget
    let search_paragraph = create_search_widget(app);

    // create temperature widget
    let temp_widget = create_temp_block(app, &icon_manager);

    // Create forecast widget
    render_forecast_widget(app, icon_manager, frame, forecast_layout);

    frame.render_widget(search_paragraph, search_layout);
    frame.render_widget(temp_widget, temp_layout);
}

fn create_search_widget<'a>(app: &mut App) -> Paragraph<'a> {
    let search_block = Block::default()
        .title("Search your city")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .border_type(BorderType::Rounded);
    let search_paragraph: Paragraph<'_> = Paragraph::new(app.search_string.to_string())
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .block(search_block);
    search_paragraph
}

fn create_temp_block<'a>(app: &mut App, icon_manager: &IconManger) -> Paragraph<'a> {
    let temperature_block = Block::default()
        .title("Temperature of your city")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .border_type(BorderType::Rounded);

    // Create the forecast string
    let forecast_string: String;

    match &app.wether {
        Some(weather) => {
            let weather_icon = icon_manager.get_icon(weather.current_conditions.icon.to_string());
            forecast_string = format!(
                "
    Weather for the city: {}
    Condition: {}
    Temp: {} °C {}
    Description: {}",
                weather.address,
                weather.current_conditions.conditions,
                weather.current_conditions.temp,
                weather_icon,
                weather.description
            );
        }
        None => {
            if app.search_string.is_empty() {
                forecast_string = format!("Kindly look up the weather forecast for your city.");
            } else {
                forecast_string = format!("Unable to get weather for {}", app.search_string);
            }
        }
    }

    let temp_paragraph = Paragraph::new(forecast_string)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .block(temperature_block);
    temp_paragraph
}

fn render_forecast_widget<'a>(
    app: &mut App,
    icon_manager: IconManger,
    frame: &mut Frame,
    area: Rect,
) {
    let forecast_block = Block::default()
        .title("Forecast of your city")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .border_type(BorderType::Rounded);

    let forecast_days = if let Some(weather) = &app.wether {
        weather.days.clone()
    } else {
        Vec::new()
    };
    if forecast_days.len() > 0 {
        let forecast_days_string: Vec<String> = forecast_days
            .iter()
            .map(|d| {
                let weather_icon = icon_manager.get_icon(d.icon.to_string());
                format!(
                    "\n\t{} | {} | Temp: {}-{} °C {} | {}\n",
                    format_date(&d.datetime),
                    d.conditions,
                    d.tempmin,
                    d.tempmax,
                    weather_icon,
                    d.description
                )
            })
            .collect();

        let list = List::new(forecast_days_string)
            .block(forecast_block)
            .style(Style::default().fg(Color::White))
            .direction(ListDirection::TopToBottom);
        frame.render_widget(list, area);
    } else {
        let forecast_paragraph = Paragraph::new("There are no forecasts")
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .block(forecast_block);
        frame.render_widget(forecast_paragraph, area);
    }
}

fn format_date(date: &str) -> String {
    let result = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    result.format("%d %B %Y").to_string()
}
