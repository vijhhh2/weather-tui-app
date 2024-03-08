use std::error;

use chrono::Days;

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
    pub search_string: String,
    pub weather_api_key: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            wether: None,
            search_string: "".to_string(),
            weather_api_key: "".to_string(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}

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
        // Construct the Url
        let start_date = chrono::Utc::now();
        let end_date = start_date.checked_add_days(Days::new(5)).unwrap();
        let format_string = "%Y-%m-%d";
        let url = format!("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/{}/{}/{}?unitGroup=metric&key={}&contentType=json", self.search_string.clone(), start_date.format(format_string), end_date.format(format_string), &self.weather_api_key);

        // Send request
        let response = reqwest::get(&url).await?;
        let status_code = response.status();
        if status_code.is_success() {
            let body = response.text().await?;
            let wether_forecast =
                serde_json::from_str::<WetherForecast>(body.as_str()).expect("Failed to prase");
            self.wether = Some(wether_forecast.clone());
        } else {
            println!("Failed {}", status_code);
        }
        Ok(())
    }
}
