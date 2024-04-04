use osm_xml::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

impl Into<geoutils::Location> for Location {
    fn into(self) -> geoutils::Location {
        geoutils::Location::new(self.lat, self.lon)
    }
}

impl From<&Node> for Location {
    fn from(value: &Node) -> Self {
        Location {
            lat: value.lat,
            lon: value.lon,
        }
    }
}

impl Location {
    pub fn fast_dist2(&self, other: &Self) -> f64 {
        let diff_lat = other.lat - self.lat;
        let diff_lon = other.lon - self.lon;

        diff_lat.powi(2) + diff_lon.powi(2)
    }
}
