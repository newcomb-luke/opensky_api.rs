use std::sync::Arc;

use log::debug;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

use crate::errors::Error;

#[derive(Debug, Serialize, Deserialize)]
/// Represents the trajectory for a certain aircraft at a given time.
pub struct FlightTrack {
    /// Unique ICAO 24-bit address of the transponder in lower case hex string representation.
    pub icao24: String,
    #[serde(alias = "startTime")]
    /// Time of the first waypoint in seconds since epoch (Unix time).
    pub start_time: f64,
    #[serde(alias = "endTime")]
    /// Time of the last waypoint in seconds since epoch (Unix time).
    pub end_time: f64,
    /// Callsign (8 characters) that holds for the whole track. Can be None.
    pub callsign: Option<String>,
    /// Waypoints of the trajectory
    pub path: Vec<Waypoint>,
}

#[derive(Debug, Serialize)]
/// Represents the single waypoint that is a basic part of flight trajectory.
pub struct Waypoint {
    /// Time which the given waypoint is associated with in seconds since epoch (Unix time).
    pub time: u64,
    /// WGS-84 latitude in decimal degrees. Can be None.
    pub latitude: Option<f64>,
    /// WGS-84 longitude in decimal degrees. Can be None.
    pub longitude: Option<f64>,
    /// Barometric altitude in meters. Can be None.
    pub baro_altitude: Option<f64>,
    /// True track in decimal degrees clockwise from north (north=0°). Can be None.
    pub true_track: Option<f64>,
    /// Boolean value which indicates if the position was retrieved from a surface position report.
    pub on_ground: bool,
}

impl<'de> Deserialize<'de> for Waypoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = Deserialize::deserialize(deserializer)?;
        match values {
            Value::Array(arr) => Ok(Waypoint::from(arr)),
            Value::Object(obj) => Ok(Waypoint::from(obj)),
            _ => Err(serde::de::Error::custom("expected array")),
        }
    }
}

impl From<Vec<Value>> for Waypoint {
    fn from(value: Vec<Value>) -> Self {
        Waypoint {
            time: value[0].as_u64().unwrap(),
            latitude: value[1].as_f64(),
            longitude: value[2].as_f64(),
            baro_altitude: value[3].as_f64(),
            true_track: value[4].as_f64(),
            on_ground: value[5].as_bool().unwrap(),
        }
    }
}

impl From<Map<String, Value>> for Waypoint {
    fn from(value: Map<String, Value>) -> Self {
        Waypoint {
            time: value["time"].as_u64().unwrap(),
            latitude: value["latitude"].as_f64(),
            longitude: value["longitude"].as_f64(),
            baro_altitude: value["baro_altitude"].as_f64(),
            true_track: value["true_track"].as_f64(),
            on_ground: value["on_ground"].as_bool().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrackRequest {
    login: Option<Arc<(String, String)>>,
    icao24: String,
    time: u64,
}

impl TrackRequest {
    pub async fn send(&self) -> Result<FlightTrack, Error> {
        let login_part = if let Some(login) = &self.login {
            format!("{}:{}@", login.0, login.1)
        } else {
            String::new()
        };

        let url = format!(
            "https://{}opensky-network.org/api/tracks/all?icao24={}&time={}",
            login_part, self.icao24, self.time
        );

        debug!("url = {}", url);

        let res = reqwest::get(url).await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let bytes = res.bytes().await?.to_vec();

                let result: FlightTrack = match serde_json::from_slice(&bytes) {
                    Ok(result) => result,
                    Err(e) => {
                        return Err(Error::InvalidJson(e));
                    }
                };

                Ok(result)
            }
            status => Err(Error::Http(status)),
        }
    }
}

pub struct TrackRequestBuilder {
    inner: TrackRequest,
}

impl TrackRequestBuilder {
    pub fn new(login: Option<Arc<(String, String)>>, icao24: String) -> Self {
        Self {
            inner: TrackRequest {
                login,
                icao24,
                time: 0,
            },
        }
    }

    pub fn at_time(&mut self, time: u64) -> &mut Self {
        self.inner.time = time;

        self
    }

    pub async fn send(self) -> Result<FlightTrack, Error> {
        self.inner.send().await
    }
}
