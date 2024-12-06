//! # OpenSky Network API
//! This is a Rust library for interacting with the OpenSky Network API.
//! The OpenSky Network is a community-based receiver network which continuously collects air traffic surveillance data.
//! Unlike other networks, OpenSky keeps the collected data forever and makes it available to researchers and developers.
//! The OpenSky Network API provides a way to access the collected data.
//! 
//! Please follow [The OpenSky Network API documentation](https://openskynetwork.github.io/opensky-api/) for more information.
//! 
//! ## Example
//! ```rust
//! use opensky_api::OpenSkyApi;
//! let api = OpenSkyApi::new();
//! let request = api.get_states().at_time(1458564121).with_icao24("3c6444".to_string());
//! match request.send().await {
//!     Ok(states) => println!("states: {:#?}", states),
//!     Err(e) => eprintln!("Error: {:?}", e),
//! }
//! ```
use std::sync::Arc;

pub mod bounding_box;
pub mod errors;
pub mod flights;
pub mod states;

use flights::FlightsRequestBuilder;
use states::StateRequestBuilder;
pub use flights::Flight;
pub use states::{States, StateVector};

#[derive(Default)]
///  The OpenSky Network API <https://openskynetwork.github.io/>
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

    /// Creates a new FlightsRequestBuilder using the given time interval. The beginning
    /// and ending times are numbers that represent times in seconds since the Unix Epoch.
    ///
    /// The interval must not span greater than 2 hours, otherwise the request will fail.
    ///
    pub fn get_flights(&self, begin: u64, end: u64) -> FlightsRequestBuilder {
        FlightsRequestBuilder::new(self.login.clone(), begin, end)
    }
}
