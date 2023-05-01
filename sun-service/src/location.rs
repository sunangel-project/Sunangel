use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}
