
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

use weather_tui::app::{App, AppResult};
use weather_tui::event::{Event, EventHandler};
use weather_tui::handler::handle_key_events;
use weather_tui::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {

    // setup logging
    // simple_logging::log_to_file("test.log", log::LevelFilter::Trace)?;

    // Create an application.
    let mut app = App::new();

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
