use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Backspace => {
            if !app.search_string.is_empty() {
                app.search_string.pop();
            }
        }
        KeyCode::Enter => {
            app.get_wether_data().await?;
        }
        KeyCode::Char(value) => {
            app.search_string.push(value);
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
