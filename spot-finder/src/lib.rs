mod direction;
pub mod location;

use std::io::Cursor;

use anyhow::anyhow;
use location::Location;
use osm_xml::{Node, OSM};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

const OVERPASS_URL: &str = "https://overpass-api.de/api/interpreter";

async fn get_osm_data(
    lower_left: Location,
    upper_right: Location,
) -> Result<String, anyhow::Error> {
    let body = format!(
        "nwr({},{},{},{})->.all;
        (
            node.all[amenity=bench];
            node.all[bench=yes];
        );
        out meta;",
        lower_left.lat, lower_left.lon, upper_right.lat, upper_right.lon,
    );

    let client = reqwest::Client::new();
    let request = client.post(OVERPASS_URL).body(body);
    let response = request.send().await?;

    if response.status() == StatusCode::OK {
        Ok(response.text().await?)
    } else {
        Err(anyhow!("overpass returned {}", response.status()))
    }
}

// Spot
#[derive(Debug, Serialize, Deserialize)]
pub struct Spot {
    pub kind: String,
    pub loc: Location,
    pub dir: Option<f64>,
}

// Searching

fn is_bench(n: &&Node) -> bool {
    n.tags
        .iter()
        .any(|t| (t.key == "amenity" && t.val == "bench") || t.key == "bench")
}

fn direction_of_node(node: &Node) -> Option<f64> {
    node.tags
        .iter()
        .find(|tag| tag.key == "direction")
        .map(|tag| tag.val.as_str())
        .map(direction::direction_from_string)
        .map(|dir| {
            if let Err(err) = &dir {
                println!("Couldn't parse direction of node {node:?}, {err}")
            }

            dir
        })
        .and_then(Result::ok)
}

pub fn search_area_short_enough(
    lower_left: Location,
    upper_right: Location,
    upper_length_limit: u32,
) -> bool {
    let lower_left: geoutils::Location = lower_left.into();
    let upper_right: geoutils::Location = upper_right.into();
    lower_left
        .distance_to(&upper_right)
        .is_ok_and(|dist| dist.meters() <= upper_length_limit.into())
}

pub async fn find_spots(
    lower_left: Location,
    upper_right: Location,
) -> Result<Vec<Spot>, anyhow::Error> {
    let osm_data = get_osm_data(lower_left, upper_right).await?;
    let osm = OSM::parse(Cursor::new(osm_data))?;

    let spots = osm
        .nodes
        .values()
        .filter(is_bench)
        .map(|node| Spot {
            kind: "bench".to_string(),
            loc: Location::from(node),
            dir: direction_of_node(node),
        })
        .collect();

    Ok(spots)
}
