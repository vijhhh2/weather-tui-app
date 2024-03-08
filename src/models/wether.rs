use serde::{Deserialize, Serialize};
use std::fmt::Display;



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WetherForecast {
    pub query_cost: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub resolved_address: String,
    pub address: String,
    pub timezone: String,
    pub tzoffset: f64,
    pub description: String,
    pub days: Vec<Day>,
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
pub struct Day {
    pub datetime: String,
    pub tempmax: f64,
    pub tempmin: f64,
    pub temp: f64,
    pub conditions: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentConditions {
    pub datetime: String,
    pub temp: f64,
    pub conditions: String,
    pub icon: String,
}