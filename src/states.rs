//! Module for searching state vectors from the OpenSky Network API.
use std::sync::Arc;

use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Map, Value};

use crate::{bounding_box::BoundingBox, errors::Error};

#[derive(Debug, Serialize)]
/// Represents a collection of state vectors returned by the OpenSky API.
pub struct States {
    pub time: u64,
    pub states: Vec<StateVector>,
}

impl<'de> Deserialize<'de> for States {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let obj: Value = Deserialize::deserialize(deserializer)?;
        let time: u64 = obj.get("time").unwrap().as_u64().unwrap();
        let states = obj.get("states").unwrap();
        let states: Vec<StateVector> = match states {
            Value::Null => Vec::new(),
            Value::Array(_) => {
                Deserialize::deserialize(states).map_err(serde::de::Error::custom)?
            }
            _ => return Err(serde::de::Error::custom("expected an array")),
        };

        Ok(States { time, states })
    }
}

#[derive(Debug, Serialize)]
/// Represents a state vector of an aircraft.
pub struct StateVector {
    /// Unique ICAO 24-bit address of the transponder in hex string
    /// representation.
    pub icao24: String,
    /// Callsign of the vehicle (8 chars). Can be None if no callsign has been
    /// received.
    pub callsign: Option<String>,
    /// Country name inferred from the ICAO 24-bit address.
    pub origin_country: String,
    /// Unix timestamp (seconds) for the last position update. Can be None if no
    /// position report was received by OpenSky within the past 15s.
    pub time_position: Option<u64>,
    /// Unix timestamp (seconds) for the last update in general. This field is
    /// updated for any new, valid message received from the transponder.
    pub last_contact: u64,
    /// WGS-84 longitude in decimal degrees.
    pub longitude: Option<f32>,
    /// WGS-84 latitude in decimal degrees.
    pub latitude: Option<f32>,
    /// Barometric altitude in meters.
    pub baro_altitude: Option<f32>,
    /// Boolean value which indicates if the position was retrieved from a
    /// surface position report.
    pub on_ground: bool,
    /// Velocity over ground in m/s.
    pub velocity: Option<f32>,
    /// True track in decimal degrees clockwise from north (north=0°). Can be
    /// None.
    pub true_track: Option<f32>,
    /// Vertical rate in m/s. A positive value indicates that the airplane is
    /// climbing, a negative value indicates that it descends.
    pub vertical_rate: Option<f32>,
    /// IDs of the receivers which contributed to this state vector. Is None if
    /// no filtering for sensor was used in the request.
    pub sensors: Option<Vec<u64>>,
    /// Geometric altitude in meters.
    pub geo_altitude: Option<f32>,
    /// The transponder code aka Squawk.
    pub squawk: Option<String>,
    /// Whether flight status indicates special purpose indicator.
    pub spi: bool,
    /// Origin of this state’s position.
    pub position_source: PositionSource,
    /// Aircraft category.
    pub category: Option<AirCraftCategory>,
}

impl<'de> Deserialize<'de> for StateVector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let all: Value = Deserialize::deserialize(deserializer)?;
        match all {
            Value::Array(arr) => Ok(StateVector::from(arr)),
            Value::Object(obj) => Ok(StateVector::from(obj)),
            _ => Err(serde::de::Error::custom("expected an array")),
        }
    }
}

impl From<Vec<Value>> for StateVector {
    fn from(value: Vec<Value>) -> Self {
        StateVector {
            icao24: from_value(value[0].clone()).unwrap(),
            callsign: from_value(value[1].clone()).unwrap(),
            origin_country: from_value(value[2].clone()).unwrap(),
            time_position: from_value(value[3].clone()).unwrap(),
            last_contact: from_value(value[4].clone()).unwrap(),
            longitude: from_value(value[5].clone()).unwrap(),
            latitude: from_value(value[6].clone()).unwrap(),
            baro_altitude: from_value(value[7].clone()).unwrap(),
            on_ground: from_value(value[8].clone()).unwrap(),
            velocity: from_value(value[9].clone()).unwrap(),
            true_track: from_value(value[10].clone()).unwrap(),
            vertical_rate: from_value(value[11].clone()).unwrap(),
            sensors: from_value(value[12].clone()).unwrap(),
            geo_altitude: from_value(value[13].clone()).unwrap(),
            squawk: from_value(value[14].clone()).unwrap(),
            spi: from_value(value[15].clone()).unwrap(),
            position_source: from_value(value[16].clone()).unwrap(),
            category: if value.len() == 18 {
                from_value(value[17].clone()).unwrap()
            } else {
                None
            },
        }
    }
}

impl From<Map<String, Value>> for StateVector {
    fn from(map: Map<String, Value>) -> Self {
        StateVector {
            icao24: from_value(map.get("icao24").unwrap().clone()).unwrap(),
            callsign: from_value(map.get("callsign").unwrap().clone()).unwrap(),
            origin_country: from_value(map.get("origin_country").unwrap().clone()).unwrap(),
            time_position: from_value(map.get("time_position").unwrap().clone()).unwrap(),
            last_contact: from_value(map.get("last_contact").unwrap().clone()).unwrap(),
            longitude: from_value(map.get("longitude").unwrap().clone()).unwrap(),
            latitude: from_value(map.get("latitude").unwrap().clone()).unwrap(),
            baro_altitude: from_value(map.get("baro_altitude").unwrap().clone()).unwrap(),
            on_ground: from_value(map.get("on_ground").unwrap().clone()).unwrap(),
            velocity: from_value(map.get("velocity").unwrap().clone()).unwrap(),
            true_track: from_value(map.get("true_track").unwrap().clone()).unwrap(),
            vertical_rate: from_value(map.get("vertical_rate").unwrap().clone()).unwrap(),
            sensors: from_value(map.get("sensors").unwrap().clone()).unwrap(),
            geo_altitude: from_value(map.get("geo_altitude").unwrap().clone()).unwrap(),
            squawk: from_value(map.get("squawk").unwrap().clone()).unwrap(),
            spi: from_value(map.get("spi").unwrap().clone()).unwrap(),
            position_source: from_value(map.get("position_source").unwrap().clone()).unwrap(),
            category: from_value(map.get("category").unwrap().clone()).unwrap(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum PositionSource {
    ADSB,
    ASTERIX,
    MLAT,
    FLARM,
}

impl<'de> Deserialize<'de> for PositionSource {
    fn deserialize<D>(deserializer: D) -> Result<PositionSource, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match Deserialize::deserialize(deserializer)? {
            Value::Number(num) => Ok(PositionSource::from(num.as_u64().unwrap() as u8)),
            Value::String(s) => Ok(PositionSource::from(s.as_str())),
            _ => Err(serde::de::Error::custom("expected a number")),
        }
    }
}

impl From<u8> for PositionSource {
    fn from(value: u8) -> Self {
        match value {
            0 => PositionSource::ADSB,
            1 => PositionSource::ASTERIX,
            2 => PositionSource::MLAT,
            3 => PositionSource::FLARM,
            _ => {
                eprintln!("unknown position source: {}", value);
                PositionSource::ADSB
            }
        }
    }
}

impl From<&str> for PositionSource {
    fn from(value: &str) -> Self {
        match value {
            "ADSB" => PositionSource::ADSB,
            "ASTERIX" => PositionSource::ASTERIX,
            "MLAT" => PositionSource::MLAT,
            "FLARM" => PositionSource::FLARM,
            _ => {
                eprintln!("unknown position source: {}", value);
                PositionSource::ADSB
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub enum AirCraftCategory {
    /// No information at all
    NoInformation,
    /// No ADS-B Emitter Category Information
    NoADSB,
    /// Light (< 15500 lbs)
    Light,
    /// Small (15500 to 75000 lbs)
    Small,
    /// Large (75000 to 300000 lbs)
    Large,
    /// High Vortex Large (aircraft such as B-757)
    HighVortexLarge,
    /// Heavy (> 300000 lbs)
    Heavy,
    /// High Performance (> 5g acceleration and 400 kts)
    HighPerformance,
    /// Rotorcraft
    Rotorcraft,
    /// Glider / sailplane
    Glider,
    /// Lighter-than-air
    LighterThanAir,
    /// Parachutist / Skydiver
    Parachutist,
    /// Ultralight / hang-glider / paraglider
    Ultralight,
    /// Reserved
    Reserved,
    /// Unmanned Aerial Vehicle
    UAV,
    /// Space / Trans-atmospheric vehicle
    Space,
    /// Surface Vehicle – Emergency Vehicle
    SurfaceEmergency,
    /// Surface Vehicle – Service Vehicle
    SurfaceService,
    /// Point Obstacle (includes tethered balloons)
    PointObstacle,
    /// Cluster Obstacle
    ClusterObstacle,
    /// Line Obstacle
    LineObstacle,
}

impl<'de> Deserialize<'de> for AirCraftCategory {
    fn deserialize<D>(deserializer: D) -> Result<AirCraftCategory, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match Deserialize::deserialize(deserializer)? {
            Value::Number(num) => Ok(AirCraftCategory::from(num.as_u64().unwrap() as u8)),
            Value::String(s) => Ok(AirCraftCategory::from(s.as_str())),
            _ => Err(serde::de::Error::custom("expected a number")),
        }
    }
}

impl From<u8> for AirCraftCategory {
    fn from(value: u8) -> Self {
        match value {
            0 => AirCraftCategory::NoInformation,
            1 => AirCraftCategory::NoADSB,
            2 => AirCraftCategory::Light,
            3 => AirCraftCategory::Small,
            4 => AirCraftCategory::Large,
            5 => AirCraftCategory::HighVortexLarge,
            6 => AirCraftCategory::Heavy,
            7 => AirCraftCategory::HighPerformance,
            8 => AirCraftCategory::Rotorcraft,
            9 => AirCraftCategory::Glider,
            10 => AirCraftCategory::LighterThanAir,
            11 => AirCraftCategory::Parachutist,
            12 => AirCraftCategory::Ultralight,
            13 => AirCraftCategory::Reserved,
            14 => AirCraftCategory::UAV,
            15 => AirCraftCategory::Space,
            16 => AirCraftCategory::SurfaceEmergency,
            17 => AirCraftCategory::SurfaceService,
            18 => AirCraftCategory::PointObstacle,
            19 => AirCraftCategory::ClusterObstacle,
            20 => AirCraftCategory::LineObstacle,
            _ => AirCraftCategory::NoInformation,
        }
    }
}

impl From<&str> for AirCraftCategory {
    fn from(value: &str) -> Self {
        match value {
            "NoInformation" => AirCraftCategory::NoInformation,
            "NoADSB" => AirCraftCategory::NoADSB,
            "Light" => AirCraftCategory::Light,
            "Small" => AirCraftCategory::Small,
            "Large" => AirCraftCategory::Large,
            "HighVortexLarge" => AirCraftCategory::HighVortexLarge,
            "Heavy" => AirCraftCategory::Heavy,
            "HighPerformance" => AirCraftCategory::HighPerformance,
            "Rotorcraft" => AirCraftCategory::Rotorcraft,
            "Glider" => AirCraftCategory::Glider,
            "LighterThanAir" => AirCraftCategory::LighterThanAir,
            "Parachutist" => AirCraftCategory::Parachutist,
            "Ultralight" => AirCraftCategory::Ultralight,
            "Reserved" => AirCraftCategory::Reserved,
            "UAV" => AirCraftCategory::UAV,
            "Space" => AirCraftCategory::Space,
            "SurfaceEmergency" => AirCraftCategory::SurfaceEmergency,
            "SurfaceService" => AirCraftCategory::SurfaceService,
            "PointObstacle" => AirCraftCategory::PointObstacle,
            "ClusterObstacle" => AirCraftCategory::ClusterObstacle,
            "LineObstacle" => AirCraftCategory::LineObstacle,
            _ => {
                eprintln!("unknown aircraft category: {}", value);
                AirCraftCategory::NoInformation
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateRequest {
    login: Option<Arc<(String, String)>>,
    bbox: Option<BoundingBox>,
    time: Option<u64>,
    icao24_addresses: Vec<String>,
    serials: Vec<u64>,
}

impl StateRequest {
    pub async fn send(&self) -> Result<States, Error> {
        let login_part = if let Some(login) = &self.login {
            format!("{}:{}@", login.0, login.1)
        } else {
            String::new()
        };

        let mut args = String::new();

        if let Some(time) = self.time {
            if args.is_empty() {
                args.push('?');
            }

            args.push_str(&format!("time={}", time));
        }

        if let Some(bbox) = self.bbox {
            if args.is_empty() {
                args.push('?');
            } else {
                args.push('&');
            }

            args.push_str(&format!(
                "lamin={}&lomin={}&lamax={}&lomax={}",
                bbox.lat_min, bbox.long_min, bbox.lat_max, bbox.long_max
            ));
        }

        if !self.icao24_addresses.is_empty() {
            if args.is_empty() {
                args.push('?');
            } else {
                args.push('&');
            }

            if let Some(first) = self.icao24_addresses.first() {
                args.push_str(&format!("icao24={}", first));
            }

            for icao24 in self.icao24_addresses.iter().skip(1) {
                args.push_str(&format!("&icao24={}", icao24));
            }
        }

        // If serial numbers are provided determines which endpoint we use
        let endpoint = if !self.serials.is_empty() {
            if args.is_empty() {
                args.push('?');
            } else {
                args.push('&');
            }

            if let Some(first) = self.serials.first() {
                args.push_str(&format!("serials={}", first));
            }

            for serial in self.serials.iter().skip(1) {
                args.push_str(&format!("&serials={}", serial));
            }

            "own"
        } else {
            "all"
        };

        let url = format!(
            "https://{}opensky-network.org/api/states/{}{}",
            login_part, endpoint, args
        );
        debug!("Request url = {}", url);

        let res = reqwest::get(url).await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let bytes = res.bytes().await?.to_vec();

                match serde_json::from_slice(&bytes) {
                    Ok(result) => Ok(result),
                    Err(err) => Err(Error::InvalidJson(err)),
                }
            }
            status => Err(Error::Http(status)),
        }
    }
}

pub struct StateRequestBuilder {
    inner: StateRequest,
}

impl StateRequestBuilder {
    pub fn new(login: Option<Arc<(String, String)>>) -> Self {
        Self {
            inner: StateRequest {
                login,
                bbox: None,
                time: None,
                icao24_addresses: Vec::new(),
                serials: Vec::new(),
            },
        }
    }

    /// Adds the provided bounding box to the request. This will only get states
    /// that are within that bounding box. This will overwrite any
    /// previously specified bounding box.
    pub fn with_bbox(mut self, bbox: BoundingBox) -> Self {
        self.inner.bbox = Some(bbox);

        self
    }

    /// Specifies the time at which to get the data. The validity of this
    /// timestamp depends on how much access the user has to historical
    /// data.
    ///
    /// This time is specified as the time in seconds since the Unix Epoch.
    pub fn at_time(mut self, timestamp: u64) -> Self {
        self.inner.time = Some(timestamp);

        self
    }

    /// Adds an ICAO24 transponder address represented by a hex string (e.g.
    /// abc9f3) to filter the request by. Calling this function multiple
    /// times will append more addresses which will be included in the
    /// returned data.
    ///
    /// If this function is never called, it will provide data for all aircraft.
    pub fn with_icao24(mut self, address: String) -> Self {
        self.inner.icao24_addresses.push(address);

        self
    }

    /// Adds a serial number of a sensor that you own. This must be owned by you
    /// and registered in order to not return an HTTP error 403 (Forbidden).
    /// Requests from your own sensors are not ratelimited.
    ///
    /// Calling this function multiple times will append more serial numbers of
    /// receiviers which provide the returned data.
    pub fn with_serial(mut self, serial: u64) -> Self {
        self.inner.serials.push(serial);

        self
    }

    /// Consumes this StateRequestBuilder and returns a new StateRequest. If
    /// this StateRequestBuilder could be used again effectively, then the
    /// finish() method should be called instead because that will allow
    /// this to be reused.
    pub fn consume(self) -> StateRequest {
        self.inner
    }

    /// Returns the StateRequest that this StateRequestBuilder has created. This
    /// clones the inner StateRequest. If this StateRequestBuilder will be
    /// only used once, the consume() method should be used instead which
    /// will only move the inner value instead of calling clone()
    pub fn finish(&self) -> StateRequest {
        self.inner.clone()
    }

    /// Consumes this StateRequestBuilder and sends the request to the API.
    pub async fn send(self) -> Result<States, Error> {
        self.inner.send().await
    }
}

impl From<StateRequestBuilder> for StateRequest {
    fn from(srb: StateRequestBuilder) -> Self {
        srb.consume()
    }
}
