use std::sync::Arc;

use serde::Deserialize;

use crate::{bounding_box::BoundingBox, errors::Error};

#[derive(Debug, Deserialize)]
struct InnerOpenSkyStates {
    pub time: u64,
    pub states: Vec<InnerStateVector>,
}

#[derive(Debug, Deserialize)]
struct ShortInnerOpenSkyStates {
    pub time: u64,
    pub states: Vec<ShortInnerStateVector>,
}

#[derive(Debug)]
pub struct OpenSkyStates {
    pub time: u64,
    pub states: Vec<StateVector>,
}

impl OpenSkyStates {
    fn from_inner(inner: InnerOpenSkyStates) -> Self {
        let mut states = Vec::with_capacity(inner.states.len());

        for inner in inner.states {
            states.push(StateVector::from_inner(inner));
        }

        Self {
            time: inner.time,
            states,
        }
    }

    fn from_short_inner(inner: ShortInnerOpenSkyStates) -> Self {
        let mut states = Vec::with_capacity(inner.states.len());

        for inner in inner.states {
            states.push(StateVector::from_short_inner(inner));
        }

        Self {
            time: inner.time,
            states,
        }
    }
}

// May Ferris forgive me.
// This needed to be done because the OpenSky API returns the state vectors as lists, and not
// objects. So this needed to be done for deserialization
#[derive(Debug, Deserialize)]
struct InnerStateVector(
    String,
    Option<String>,
    String,
    Option<u64>,
    u64,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    bool,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<Vec<u64>>,
    Option<f32>,
    Option<String>,
    bool,
    u8,
    u32,
);

// I am very close to writing my own Json parser, because serde_json does not seem to be extremely
// well made for deserializing things that act this way. This is required, because in certain API
// accesses, the last undocumented 17th field is actually not provided. This will probably be
// temporary, but is so far required.
#[derive(Debug, Deserialize)]
struct ShortInnerStateVector(
    String,
    Option<String>,
    String,
    Option<u64>,
    u64,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    bool,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<Vec<u64>>,
    Option<f32>,
    Option<String>,
    bool,
    u8,
);

#[derive(Debug)]
pub struct StateVector {
    pub icao24: String,
    pub callsign: Option<String>,
    pub origin_country: String,
    pub time_position: Option<u64>,
    pub last_contact: u64,
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
    pub baro_altitude: Option<f32>,
    pub on_ground: bool,
    pub velocity: Option<f32>,
    pub true_track: Option<f32>,
    pub vertical_rate: Option<f32>,
    pub sensors: Option<Vec<u64>>,
    pub geo_altitude: Option<f32>,
    pub squawk: Option<String>,
    pub spi: bool,
    pub position_source: u8,
    /// There is an undocumented extra field in StateVectors, for now it will be read, and just
    /// ignored. This will be updated when the API reference begins to list this field
    pub undocumented: Option<u32>,
}

impl StateVector {
    fn from_inner(isv: InnerStateVector) -> Self {
        Self {
            icao24: isv.0,
            callsign: isv.1,
            origin_country: isv.2,
            time_position: isv.3,
            last_contact: isv.4,
            longitude: isv.5,
            latitude: isv.6,
            baro_altitude: isv.7,
            on_ground: isv.8,
            velocity: isv.9,
            true_track: isv.10,
            vertical_rate: isv.11,
            sensors: isv.12,
            geo_altitude: isv.13,
            squawk: isv.14,
            spi: isv.15,
            position_source: isv.16,
            undocumented: Some(isv.17),
        }
    }

    fn from_short_inner(isv: ShortInnerStateVector) -> Self {
        Self {
            icao24: isv.0,
            callsign: isv.1,
            origin_country: isv.2,
            time_position: isv.3,
            last_contact: isv.4,
            longitude: isv.5,
            latitude: isv.6,
            baro_altitude: isv.7,
            on_ground: isv.8,
            velocity: isv.9,
            true_track: isv.10,
            vertical_rate: isv.11,
            sensors: isv.12,
            geo_altitude: isv.13,
            squawk: isv.14,
            spi: isv.15,
            position_source: isv.16,
            undocumented: None,
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
    pub async fn send(&self) -> Result<OpenSkyStates, Error> {
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

            if let Some(first) = self.icao24_addresses.get(0) {
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

            if let Some(first) = self.serials.get(0) {
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

                Ok(if self.time.is_some() {
                    let short_inner_states: ShortInnerOpenSkyStates =
                        serde_json::from_slice(&bytes)?;

                    OpenSkyStates::from_short_inner(short_inner_states)
                } else {
                    // Retry for random API deviations
                    if let Ok(inner_states) = serde_json::from_slice(&bytes) {
                        OpenSkyStates::from_inner(inner_states)
                    } else {
                        let short_inner_states: ShortInnerOpenSkyStates =
                            serde_json::from_slice(&bytes)?;

                        OpenSkyStates::from_short_inner(short_inner_states)
                    }
                })
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
    pub async fn send(self) -> Result<OpenSkyStates, Error> {
        self.inner.send().await
    }
}

impl From<StateRequestBuilder> for StateRequest {
    fn from(srb: StateRequestBuilder) -> Self {
        srb.consume()
    }
}
