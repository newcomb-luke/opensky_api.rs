//! # OpenSky Network API
//! This is a Rust library for interacting with the OpenSky Network API.
//! The OpenSky Network is a community-based receiver network which continuously
//! collects air traffic surveillance data. Unlike other networks, OpenSky keeps
//! the collected data forever and makes it available to researchers and
//! developers. The OpenSky Network API provides a way to access the collected
//! data.
//!
//! Please follow [The OpenSky Network API documentation](https://openskynetwork.github.io/opensky-api/) for more information.
//!
//! ## Example
//!
//! Get the state vectors of aircraft.
//! ```rust
//! use opensky_api::OpenSkyApi;
//! #[tokio::main]
//! async fn main() {
//!     let api = OpenSkyApi::new();
//!     let request = api
//!         .get_states()
//!         .at_time(1458564121)
//!         .with_icao24("3c6444".to_string());
//!     let result = request.send().await.expect("Failed to get states");
//!     println!("{:#?}", result);
//! }
//! ```
//!
//! Get the flight data of aircraft.
//! ```rust
//! use opensky_api::OpenSkyApi;
//! use std::env;
//! #[tokio::main]
//! async fn main() {
//!     dotenv::dotenv().ok();
//!     // setup OPENSKY_USER and OPENSKY_PASS in .env file
//!     let username = env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
//!     let password = env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");
//!     let api = OpenSkyApi::with_login(username, password);
//!
//!     let now = std::time::SystemTime::now()
//!         .duration_since(std::time::UNIX_EPOCH)
//!         .unwrap()
//!         .as_secs();
//!     let mut request = api.get_flights(now - 7 * 24 * 60 * 60, now);
//!     request.by_aircraft("8990ed".to_string());
//!     let result = request.send().await.expect("Failed to get flights");
//!     println!("{:#?}", result);
//! }
//! ```
use std::sync::Arc;

pub mod bounding_box;
pub mod errors;
pub mod flights;
pub mod states;
pub mod tracks;

pub use bounding_box::BoundingBox;
pub use flights::Flight;
use flights::FlightsRequestBuilder;
use states::StateRequestBuilder;
pub use states::{StateVector, States};
use tracks::TrackRequestBuilder;
pub use tracks::{FlightTrack, Waypoint};

#[derive(Default)]
///  The OpenSky Network API <https://openskynetwork.github.io/opensky-api>
pub struct OpenSkyApi {
    login: Option<Arc<(String, String)>>,
}

impl OpenSkyApi {
    /// Creates a new anonymous OpenSkyApi instance
    pub fn new() -> Self {
        Self { login: None }
    }

    /// Creates a new OpenSkyApi instance with the provided username and
    /// password
    pub fn with_login(username: String, password: String) -> Self {
        Self {
            login: Some(Arc::new((username, password))),
        }
    }

    /// Creates a new StateRequestBuilder which can be used to create
    /// StateRequests
    pub fn get_states(&self) -> StateRequestBuilder {
        StateRequestBuilder::new(self.login.clone())
    }

    /// Creates a new FlightsRequestBuilder using the given time interval. The
    /// beginning and ending times are numbers that represent times in
    /// seconds since the Unix Epoch.
    ///
    /// The interval must not span greater than 2 hours, otherwise the request
    /// will fail.
    pub fn get_flights(&self, begin: u64, end: u64) -> FlightsRequestBuilder {
        FlightsRequestBuilder::new(self.login.clone(), begin, end)
    }

    /// Create a new TrackRequestBuilder for the given icao24 address of a
    /// certain aircraft.
    ///
    /// In contrast to state vectors, trajectories do not contain all
    /// information we have about the flight, but rather show the aircraft’s
    /// general movement pattern. For this reason, waypoints are selected
    /// among available state vectors given the following set of rules:
    /// * The first point is set immediately after the the aircraft’s expected
    ///   departure, or after the network received the first position when the
    ///   aircraft entered its reception range.
    /// * The last point is set right before the aircraft’s expected arrival, or
    ///   the aircraft left the networks reception range.
    /// * There is a waypoint at least every 15 minutes when the aircraft is
    ///   in-flight.
    /// * A waypoint is added if the aircraft changes its track more than 2.5°.
    /// * A waypoint is added if the aircraft changes altitude by more than 100m
    ///   (~330ft).
    /// * A waypoint is added if the on-ground state changes.
    ///
    /// Tracks are strongly related to flights. Internally, we compute flights
    /// and tracks within the same processing step. As such, it may be
    /// beneficial to retrieve a list of flights with the API methods from
    /// above, and use these results with the give time stamps to retrieve
    /// detailed track information.
    pub fn get_tracks(&self, icao24: String) -> TrackRequestBuilder {
        TrackRequestBuilder::new(self.login.clone(), icao24)
    }
}
