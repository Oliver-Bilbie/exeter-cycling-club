use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Route {
    pub status: String,
    pub id: String,
    pub name: String,
    pub message: String,
    pub distance: String,
    pub elevation_gain: String,
    pub map_url: String,
    pub is_private: String,
}

impl Route {
    pub fn cancelled(message: String) -> Self {
        Self {
            status: "cancelled".to_string(),
            id: String::new(),
            name: String::new(),
            message,
            distance: String::new(),
            elevation_gain: String::new(),
            map_url: String::new(),
            is_private: "false".to_string(),
        }
    }

    pub fn unavailable() -> Self {
        Self {
            status: "unavailable".to_string(),
            id: String::new(),
            name: String::new(),
            message: "Subscribe to email updates to find out when a route is announced".to_string(),
            distance: String::new(),
            elevation_gain: String::new(),
            map_url: String::new(),
            is_private: "false".to_string(),
        }
    }

    pub fn is_private(&self) -> bool {
        self.is_private == "true"
    }

    pub fn is_ready(&self) -> bool {
        self.status == "ready"
    }
}
