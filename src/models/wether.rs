use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WetherForecast {
    pub query_cost: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub resolved_address: String,
    pub address: String,
    pub timezone: String,
    pub tzoffset: f64,
    pub description: String,
    pub current_conditions: CurrentConditions,
}

impl Display for WetherForecast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let response = serde_json::to_string(self);
        match response {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "{}", self.address),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentConditions {
    pub datetime: String,
    pub datetime_epoch: f64,
    pub temp: f64,
    pub feelslike: f64,
    pub humidity: f64,
    pub dew: f64,
    pub precip: f64,
    pub precipprob: f64,
    pub snow: f64,
    pub snowdepth: f64,
    pub preciptype: Value,
    pub windgust: f64,
    pub windspeed: f64,
    pub winddir: f64,
    pub pressure: f64,
    pub visibility: f64,
    pub cloudcover: f64,
    pub solarradiation: f64,
    pub solarenergy: f64,
    pub uvindex: f64,
    pub severerisk: f64,
    pub conditions: String,
    pub icon: String,
    pub stations: Vec<Value>,
    pub source: String,
    pub sunrise: String,
    pub sunrise_epoch: f64,
    pub sunset: String,
    pub sunset_epoch: f64,
    pub moonphase: f64,
}
