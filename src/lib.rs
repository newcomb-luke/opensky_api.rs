use serde::Deserialize;
use std::sync::Arc;

pub mod errors;
use errors::Error;

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

/*
struct InnerFlight(
    String,
    u64,
    Option<String>,
    u64,
    Option<String>,
    Option<String>,
    u32,
    u32,
    u32,
    u32,
    u16,
    u16,
);

#[derive(Debug, Deserialize)]
pub struct Flight {
    pub icao24: String,
    #[serde(rename(deserialize = "firstSeen"))]
    pub first_seen: u64,
    #[serde(rename(deserialize = "firstSeen"))]
    pub est_departure_airport: Option<String>,
    #[serde(rename(deserialize = "firstSeen"))]
    pub last_seen: u64,
    #[serde(rename(deserialize = "firstSeen"))]
    pub est_arrival_airport: Option<String>,
    pub callsign: Option<String>,
    #[serde(rename(deserialize = "firstSeen"))]
    pub est_departure_airport_horiz_distance: u32,
    #[serde(rename(deserialize = "firstSeen"))]
    pub est_departure_airport_vert_distance: u32,
    #[serde(rename(deserialize = "firstSeen"))]
    pub est_arrival_airport_horiz_distance: u32,
    #[serde(rename(deserialize = "firstSeen"))]
    pub est_arrival_airport_vert_distance: u32,
    #[serde(rename(deserialize = "firstSeen"))]
    pub departure_airport_candidates_count: u16,
    #[serde(rename(deserialize = "firstSeen"))]
    pub arrival_airport_candidates_count: u16,
}

impl Flight {
    fn from_inner(i_f: InnerFlight) -> Self {
        Self {
            icao24: i_f.0,
            first_seen: i_f.1,
            est_departure_airport: i_f.2,
            last_seen: i_f.3,
            est_arrival_airport: i_f.4,
            callsign: i_f.5,
            est_departure_airport_horiz_distance: i_f.6,
            est_departure_airport_vert_distance: i_f.7,
            est_arrival_airport_horiz_distance: i_f.8,
            est_arrival_airport_vert_distance: i_f.9,
            departure_airport_candidates_count: i_f.10,
            arrival_airport_candidates_count: i_f.11,
        }
    }
}
*/

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

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub lat_min: f32,
    pub lat_max: f32,
    pub long_min: f32,
    pub long_max: f32,
}

impl BoundingBox {
    pub fn new(lat_min: f32, lat_max: f32, long_min: f32, long_max: f32) -> Self {
        Self {
            lat_min,
            lat_max,
            long_min,
            long_max,
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

#[derive(Debug, Clone)]
struct FlightsRequest {
    login: Option<Arc<(String, String)>>,
    begin: u64,
    end: u64,
    icao24_address: Option<String>,
}

#[derive(Debug, Clone)]
struct ArrivalsRequest {}

/*
impl FlightsRequest {
    pub async fn send(&self) -> Result<(), Error> {
        let login_part = if let Some(login) = &self.login {
            format!("{}:{}@", login.0, login.1)
        } else {
            String::new()
        };

        let mut args = String::new();

        args.push_str(&format!("?begin={}&end={}", self.begin, self.end));

        let endpoint = "all";

        let url = format!(
            "https://{}opensky-network.org/api/flights/{}{}",
            login_part, endpoint, args
        );

        println!("url = {}", url);

        let res = reqwest::get(url).await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let bytes = res.bytes().await?.to_vec();

                let string = String::from_utf8(bytes).unwrap();

                println!("{}", &string[0..200]);

                unimplemented!();

                Ok(())
            }
            status => Err(Error::Http(status)),
        }
    }
}
*/

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

/*
struct FlightsRequestBuilder {
    inner: FlightsRequest,
}

impl FlightsRequestBuilder {
    fn new(login: Option<Arc<(String, String)>>, begin: u64, end: u64) -> Self {
        Self {
            inner: FlightsRequest {
                login,
                begin,
                end,
                icao24_address: None,
            },
        }
    }

    /// This method is redundant, but can be used to reuse the same FlightsRequestBuilder multiple
    /// times to create different requests. This sets the beginning and end of the flight request
    /// interval. The beginning and ending times are numbers that represent times in seconds since
    /// the Unix Epoch.
    ///
    /// The interval must not span greater than 2 hours, otherwise the request will fail.
    ///
    pub fn in_interval(&mut self, begin: u64, end: u64) -> &mut Self {
        self.inner.begin = begin;
        self.inner.end = end;

        self
    }

    /// This method can be used to filter the flight data by a specific aircraft. The aircraft
    /// ICAO24 address is in hex string representation.
    ///
    pub fn by_aircraft(&mut self, address: String) -> &mut Self {
        self.inner.icao24_address = Some(address);

        self
    }

    /// Consumes this FlightsRequestBuilder and returns a new FlightsRequest. If this
    /// FlightsRequestBuilder could be used again effectively, then the finish() method should
    /// be called instead because that will allow this to be reused.
    ///
    pub fn consume(self) -> FlightsRequest {
        self.inner
    }

    /// Returns the FlightsRequest that this FlightsRequestBuilder has created. This clones the inner
    /// FlightsRequest. If this FlightsRequestBuilder will be only used once, the consume() method
    /// should be used instead which will only move the inner value instead of calling clone()
    ///
    pub fn finish(&self) -> FlightsRequest {
        self.inner.clone()
    }

    /// Consumes this FlightsRequestBuilder and sends the request to the API.
    pub async fn send(self) -> Result<(), Error> {
        self.inner.send().await
    }
}
*/

impl StateRequestBuilder {
    fn new(login: Option<Arc<(String, String)>>) -> Self {
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

pub struct OpenSkyApi {
    login: Option<Arc<(String, String)>>,
}

impl OpenSkyApi {
    /// Creates a new anonymous OpenSkyApi instance
    pub fn new() -> Self {
        Self { login: None }
    }

    /// Creates a new OpenSkyApi instance with the provided username and password
    pub fn with_login(username: String, password: String) -> Self {
        Self {
            login: Some(Arc::new((username, password))),
        }
    }

    /// Creates a new StateRequestBuilder which can be used to create StateRequests
    pub fn get_states(&self) -> StateRequestBuilder {
        StateRequestBuilder::new(self.login.clone())
    }

    /*
    /// Creates a new FlightsRequestBuilder using the given time interval. The beginning
    /// and ending times are numbers that represent times in seconds since the Unix Epoch.
    ///
    /// The interval must not span greater than 2 hours, otherwise the request will fail.
    ///
    pub fn get_flights(&self, begin: u64, end: u64) -> FlightsRequestBuilder {
        FlightsRequestBuilder::new(self.login.clone(), begin, end)
    }
    */
}

impl From<StateRequestBuilder> for StateRequest {
    fn from(srb: StateRequestBuilder) -> Self {
        srb.consume()
    }
}

/*
impl From<FlightsRequestBuilder> for FlightsRequest {
    fn from(frb: FlightsRequestBuilder) -> Self {
        frb.consume()
    }
}
*/
