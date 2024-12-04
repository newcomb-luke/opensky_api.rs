use std::sync::Arc;

use crate::errors::Error;
use serde::Deserialize;

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

#[derive(Debug, Clone)]
struct FlightsRequest {
    login: Option<Arc<(String, String)>>,
    begin: u64,
    end: u64,
    icao24_address: Option<String>,
}

#[derive(Debug, Clone)]
struct ArrivalsRequest {}

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


pub struct FlightsRequestBuilder {
    inner: FlightsRequest,
}

impl FlightsRequestBuilder {
    pub fn new(login: Option<Arc<(String, String)>>, begin: u64, end: u64) -> Self {
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

impl From<FlightsRequestBuilder> for FlightsRequest {
    fn from(frb: FlightsRequestBuilder) -> Self {
        frb.consume()
    }
}
