use clap::Parser;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

use weather_tui::app::{App, AppResult};
use weather_tui::event::{Event, EventHandler};
use weather_tui::handler::handle_key_events;
use weather_tui::tui::Tui;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    weather_api_key: String,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // setup logging
    let args = Args::parse();

    // Create an application.
    let mut app = App::new();
    app.weather_api_key = args.weather_api_key.clone();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app).await?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
