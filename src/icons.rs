use std::collections::HashMap;

pub struct IconManger {
    icons: HashMap<String, String>,
}

impl IconManger {
    pub fn new() -> Self {
        let mut icons = HashMap::new();
        icons.insert("snow".to_string(), " ".to_string());
        icons.insert("snow-showers-day".to_string(), " ".to_string());
        icons.insert("snow-showers-night".to_string(), " ".to_string());
        icons.insert("rain".to_string(), " ".to_string());
        icons.insert("thunder-rain".to_string(), " ".to_string());
        icons.insert("thunder-showers-day".to_string(), " ".to_string());
        icons.insert("thunder-showers-night".to_string(), " ".to_string());
        icons.insert("showers-day".to_string(), " ".to_string());
        icons.insert("showers-night".to_string(), " ".to_string());
        icons.insert("fog".to_string(), " ".to_string());
        icons.insert("wind".to_string(), " ".to_string());
        icons.insert("cloudy".to_string(), " ".to_string());
        icons.insert("partly-cloudy-day".to_string(), "󰖕 ".to_string());
        icons.insert("partly-cloudy-night".to_string(), " ".to_string());
        icons.insert("clear-day".to_string(), " ".to_string());
        icons.insert("clear-night".to_string(), " ".to_string());
        Self {
            icons
        }
    }

    pub fn get_icon(&self, name: String) -> String {
        match &self.icons.get(name.as_str()) {
            Some(icon) => {
                icon.to_string()
            },
            None => "?".to_string(),
        }
    }
}