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
