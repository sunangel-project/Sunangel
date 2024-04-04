use osm_xml::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

impl From<Location> for geoutils::Location {
    fn from(value: Location) -> Self {
        Self::new(value.lat, value.lon)
    }
}

impl From<geoutils::Location> for Location {
    fn from(value: geoutils::Location) -> Self {
        Self {
            lat: value.latitude(),
            lon: value.longitude(),
        }
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

    pub fn center(a: &Self, b: &Self) -> Self {
        let a: geoutils::Location = a.clone().into();
        let b: geoutils::Location = b.clone().into();

        geoutils::Location::center(&[&a, &b]).into()
    }
}
