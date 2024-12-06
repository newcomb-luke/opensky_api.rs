//! Contains the structs and functions for getting state vectors from the OpenSky API.
use std::sync::Arc;

use log::{debug, info, warn};
use serde::Deserialize;
use serde_json::{from_value, Value};

use crate::{bounding_box::BoundingBox, errors::Error};

#[derive(Debug, Deserialize)]
/// Represents a collection of state vectors returned by the OpenSky API.
pub struct States {
    pub time: u64,
    pub states: Vec<StateVector>,
}

#[derive(Debug)]
/// Represents a state vector of an aircraft.
pub struct StateVector {
    /// Unique ICAO 24-bit address of the transponder in hex string representation.
    pub icao24: String,
    /// Callsign of the vehicle (8 chars). Can be null if no callsign has been received.
    pub callsign: Option<String>,
    /// Country name inferred from the ICAO 24-bit address.
    pub origin_country: String,
    /// Unix timestamp (seconds) for the last position update. Can be null if no position report was received by OpenSky within the past 15s.
    pub time_position: Option<u64>,
    /// Unix timestamp (seconds) for the last update in general. This field is updated for any new, valid message received from the transponder.
    pub last_contact: u64,
    /// WGS-84 longitude in decimal degrees. Can be null.
    pub longitude: Option<f32>,
    /// WGS-84 latitude in decimal degrees. Can be null.
    pub latitude: Option<f32>,
    /// Barometric altitude in meters. Can be null.
    pub baro_altitude: Option<f32>,
    /// Boolean value which indicates if the position was retrieved from a surface position report.
    pub on_ground: bool,
    /// Velocity over ground in m/s. Can be null.
    pub velocity: Option<f32>,
    /// True track in decimal degrees clockwise from north (north=0°). Can be null.
    pub true_track: Option<f32>,
    /// Vertical rate in m/s. A positive value indicates that the airplane is climbing, a negative value indicates that it descends. Can be null.
    pub vertical_rate: Option<f32>,
    /// IDs of the receivers which contributed to this state vector. Is null if no filtering for sensor was used in the request.
    pub sensors: Option<Vec<u64>>,
    /// Geometric altitude in meters. Can be null.
    pub geo_altitude: Option<f32>,
    /// The transponder code aka Squawk. Can be null.
    pub squawk: Option<String>,
    /// Whether flight status indicates special purpose indicator.
    pub spi: bool,
    /// Origin of this state’s position.
    /// 0 = ADS-B
    /// 1 = ASTERIX
    /// 2 = MLAT
    /// 3 = FLARM
    pub position_source: u8,
    /// Aircraft category.
    /// 0 = No information at all
    /// 1 = No ADS-B Emitter Category Information
    /// 2 = Light (< 15500 lbs)
    /// 3 = Small (15500 to 75000 lbs)
    /// 4 = Large (75000 to 300000 lbs)
    /// 5 = High Vortex Large (aircraft such as B-757)
    /// 6 = Heavy (> 300000 lbs)
    /// 7 = High Performance (> 5g acceleration and 400 kts)
    /// 8 = Rotorcraft
    /// 9 = Glider / sailplane
    /// 10 = Lighter-than-air
    /// 11 = Parachutist / Skydiver
    /// 12 = Ultralight / hang-glider / paraglider
    /// 13 = Reserved
    /// 14 = Unmanned Aerial Vehicle
    /// 15 = Space / Trans-atmospheric vehicle
    /// 16 = Surface Vehicle – Emergency Vehicle
    /// 17 = Surface Vehicle – Service Vehicle
    /// 18 = Point Obstacle (includes tethered balloons)
    /// 19 = Cluster Obstacle
    /// 20 = Line Obstacle
    pub category: Option<u32>,
}

impl<'de> Deserialize<'de> for StateVector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let values: Vec<Value> = Deserialize::deserialize(deserializer)?;

        if values.len() != 18 && values.len() != 17 {
            warn!("expected 18 elements, got {}", values.len());
            return Err(serde::de::Error::invalid_length(
                values.len(),
                &"expected 18 elements",
            ));
        }

        Ok(StateVector {
            icao24: from_value(values[0].clone()).map_err(serde::de::Error::custom)?,
            callsign: from_value(values[1].clone()).map_err(serde::de::Error::custom)?,
            origin_country: from_value(values[2].clone()).map_err(serde::de::Error::custom)?,
            time_position: from_value(values[3].clone()).map_err(serde::de::Error::custom)?,
            last_contact: from_value(values[4].clone()).map_err(serde::de::Error::custom)?,
            longitude: from_value(values[5].clone()).map_err(serde::de::Error::custom)?,
            latitude: from_value(values[6].clone()).map_err(serde::de::Error::custom)?,
            baro_altitude: from_value(values[7].clone()).map_err(serde::de::Error::custom)?,
            on_ground: from_value(values[8].clone()).map_err(serde::de::Error::custom)?,
            velocity: from_value(values[9].clone()).map_err(serde::de::Error::custom)?,
            true_track: from_value(values[10].clone()).map_err(serde::de::Error::custom)?,
            vertical_rate: from_value(values[11].clone()).map_err(serde::de::Error::custom)?,
            sensors: from_value(values[12].clone()).map_err(serde::de::Error::custom)?,
            geo_altitude: from_value(values[13].clone()).map_err(serde::de::Error::custom)?,
            squawk: from_value(values[14].clone()).map_err(serde::de::Error::custom)?,
            spi: from_value(values[15].clone()).map_err(serde::de::Error::custom)?,
            position_source: from_value(values[16].clone()).map_err(serde::de::Error::custom)?,
            category: if values.len() == 18 {
                from_value(values[17].clone()).map_err(serde::de::Error::custom)?
            } else {
                None
            },
        })
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

        let res = reqwest::get(url).await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let bytes = res.bytes().await?.to_vec();

                let time = self.time.unwrap_or_default();
                info!("received: {:#?}", String::from_utf8_lossy(&bytes));
                let states: States = match serde_json::from_slice(&bytes) {
                    Ok(result) => result,
                    Err(err) => {
                        warn!("JSON Error: {}", err);
                        if err.to_string().as_str().starts_with("invalid type: null") {
                            States {
                                time,
                                states: Vec::new(),
                            }
                        } else {
                            return Err(Error::InvalidJson(err));
                        }
                    }
                };

                debug!("ShortInnerOpenSkyStates: \n{:#?}", states);

                Ok(states)
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

    /// Adds the provided bounding box to the request. This will only get states that are within
    /// that bounding box. This will overwrite any previously specified bounding box.
    ///
    pub fn with_bbox(mut self, bbox: BoundingBox) -> Self {
        self.inner.bbox = Some(bbox);

        self
    }

    /// Specifies the time at which to get the data. The validity of this timestamp depends on how
    /// much access the user has to historical data.
    ///
    /// This time is specified as the time in seconds since the Unix Epoch.
    ///
    pub fn at_time(mut self, timestamp: u64) -> Self {
        self.inner.time = Some(timestamp);

        self
    }

    /// Adds an ICAO24 transponder address represented by a hex string (e.g. abc9f3) to filter the
    /// request by. Calling this function multiple times will append more addresses which will be
    /// included in the returned data.
    ///
    /// If this function is never called, it will provide data for all aircraft.
    ///
    pub fn with_icao24(mut self, address: String) -> Self {
        self.inner.icao24_addresses.push(address);

        self
    }

    /// Adds a serial number of a sensor that you own. This must be owned by you and registered in
    /// order to not return an HTTP error 403 (Forbidden). Requests from your own sensors are not
    /// ratelimited.
    ///
    /// Calling this function multiple times will append more serial numbers of receiviers which
    /// provide the returned data.
    ///
    pub fn with_serial(mut self, serial: u64) -> Self {
        self.inner.serials.push(serial);

        self
    }

    /// Consumes this StateRequestBuilder and returns a new StateRequest. If this
    /// StateRequestBuilder could be used again effectively, then the finish() method should
    /// be called instead because that will allow this to be reused.
    ///
    pub fn consume(self) -> StateRequest {
        self.inner
    }

    /// Returns the StateRequest that this StateRequestBuilder has created. This clones the inner
    /// StateRequest. If this StateRequestBuilder will be only used once, the consume() method
    /// should be used instead which will only move the inner value instead of calling clone()
    ///
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
