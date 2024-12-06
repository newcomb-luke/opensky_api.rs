//! This module contains the FlightsRequest and FlightsRequestBuilder structs which are used to
use std::sync::Arc;

use crate::errors::Error;
use log::debug;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Represents a flight object returned by the OpenSky API
pub struct Flight {
    /// Unique ICAO 24-bit address of the transponder in hex string representation. All letters are lower case.
    pub icao24: String,
    #[serde(rename(deserialize = "firstSeen"))]
    /// Estimated time of departure for the flight as Unix time (seconds since epoch).
    pub first_seen: u64,
    #[serde(rename(deserialize = "estDepartureAirport"))]
    /// ICAO code of the estimated departure airport. Can be null if the airport could not be identified.
    pub est_departure_airport: Option<String>,
    #[serde(rename(deserialize = "lastSeen"))]
    /// Estimated time of arrival for the flight as Unix time (seconds since epoch).
    pub last_seen: u64,
    #[serde(rename(deserialize = "estArrivalAirport"))]
    ///  ICAO code of the estimated arrival airport. Can be null if the airport could not be identified.
    pub est_arrival_airport: Option<String>,
    /// Callsign of the vehicle (8 chars). Can be null if no callsign has been received. If the vehicle transmits multiple callsigns during the flight, we take the one seen most frequently.
    pub callsign: Option<String>,
    #[serde(rename(deserialize = "estDepartureAirportHorizDistance"))]
    /// Horizontal distance of the last received airborne position to the estimated departure airport in meters.
    pub est_departure_airport_horiz_distance: Option<u32>,
    #[serde(rename(deserialize = "estDepartureAirportVertDistance"))]
    /// Vertical distance of the last received airborne position to the estimated departure airport in meters.
    pub est_departure_airport_vert_distance: Option<u32>,
    #[serde(rename(deserialize = "estArrivalAirportHorizDistance"))]
    /// Horizontal distance of the last received airborne position to the estimated arrival airport in meters.
    pub est_arrival_airport_horiz_distance: Option<u32>,
    #[serde(rename(deserialize = "estArrivalAirportVertDistance"))]
    /// Vertical distance of the last received airborne position to the estimated arrival airport in meters.
    pub est_arrival_airport_vert_distance: Option<u32>,
    #[serde(rename(deserialize = "departureAirportCandidatesCount"))]
    /// Number of other possible departure airports. These are airports in short distance to estDepartureAirport.
    pub departure_airport_candidates_count: u16,
    #[serde(rename(deserialize = "arrivalAirportCandidatesCount"))]
    /// Number of other possible departure airports.
    pub arrival_airport_candidates_count: u16,
}

#[derive(Debug, Clone)]
enum FlightsRequestType {
    All,
    Aircraft(String),
    Arrival(String),
    Departure(String),
}
#[derive(Debug, Clone)]
pub struct FlightsRequest {
    login: Option<Arc<(String, String)>>,
    begin: u64,
    end: u64,
    request_type: FlightsRequestType,
}

impl FlightsRequest {
    pub async fn send(&self) -> Result<Vec<Flight>, Error> {
        let login_part = if let Some(login) = &self.login {
            format!("{}:{}@", login.0, login.1)
        } else {
            String::new()
        };

        let endpoint = match &self.request_type {
            FlightsRequestType::All => "all",
            FlightsRequestType::Aircraft(_) => "aircraft",
            FlightsRequestType::Arrival(_) => "arrival",
            FlightsRequestType::Departure(_) => "departure",
        };

        let mut args = format!("?begin={}&end={}", self.begin, self.end);
        let additional_filters = match &self.request_type {
            FlightsRequestType::All => String::new(),
            FlightsRequestType::Aircraft(address) => format!("&icao24={}", address),
            FlightsRequestType::Arrival(airport_icao) => format!("&airport={}", airport_icao),
            FlightsRequestType::Departure(airport_icao) => format!("&airport={}", airport_icao),
        };
        args.push_str(&additional_filters);

        let url = format!(
            "https://{}opensky-network.org/api/flights/{}{}",
            login_part, endpoint, args
        );

        debug!("url = {}", url);

        let res = reqwest::get(url).await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let bytes = res.bytes().await?.to_vec();

                let result: Vec<Flight> = match serde_json::from_slice(&bytes) {
                    Ok(result) => result,
                    Err(e) => {
                        debug!("Error: {:?}", e);
                        return Err(Error::InvalidJson(e));
                    }
                };

                Ok(result)
            }
            status => Err(Error::Http(status)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlightsRequestBuilder {
    inner: FlightsRequest,
}

impl FlightsRequestBuilder {
    pub fn new(login: Option<Arc<(String, String)>>, begin: u64, end: u64) -> Self {
        //assert!(end - begin <= 7200, "Interval must not span greater than 2 hours");
        //assert!(end > begin, "End time must be greater than begin time");
        Self {
            inner: FlightsRequest {
                login,
                begin,
                end,
                request_type: FlightsRequestType::All,
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
        assert!(end - begin <= 7200, "Interval must not span greater than 2 hours");
        assert!(end > begin, "End time must be greater than begin time");
        self.inner.begin = begin;
        self.inner.end = end;

        self
    }

    /// This method can be used to filter the flight data by a specific aircraft. The aircraft
    /// ICAO24 address is in hex string representation.
    ///
    pub fn by_aircraft(&mut self, address: String) -> &mut Self {
        self.inner.request_type = FlightsRequestType::Aircraft(address);

        self
    }

    /// This method can be used to filter the flight data by a arrival airport. The airport
    /// ICAO code is a 4-letter string.
    /// 
    pub fn by_arrival(&mut self, airport_icao: String) -> &mut Self {
        self.inner.request_type = FlightsRequestType::Arrival(airport_icao);

        self
    }

    /// This method can be used to filter the flight data by departure airport.
    /// 
    pub fn by_departure(&mut self, airport_icao: String) -> &mut Self {
        self.inner.request_type = FlightsRequestType::Departure(airport_icao);

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
    pub async fn send(self) -> Result<Vec<Flight>, Error> {
        self.inner.send().await
    }
}

impl From<FlightsRequestBuilder> for FlightsRequest {
    fn from(frb: FlightsRequestBuilder) -> Self {
        frb.consume()
    }
}
