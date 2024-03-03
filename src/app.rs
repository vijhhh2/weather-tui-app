use std::error;
use log::debug;

use crate::models::wether::WetherForecast;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub wether: Option<WetherForecast>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            wether: None,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub async fn get_wether_data(&mut self) -> AppResult<()> {
        let url = "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/anantapur?unitGroup=metric&key=EVUYUMR4QS6Y8Y9TGW977FSQK&contentType=json";
        let body = reqwest::get(url).await?.text().await?;
        let wether_forecast = serde_json::from_str::<WetherForecast>(body.as_str()).expect("Failed to prase");
        self.wether = Some(wether_forecast.clone());
        Ok(())
    }
}
