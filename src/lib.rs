use std::sync::Arc;

pub mod bounding_box;
pub mod errors;
pub mod flights;
pub mod states;

use flights::FlightsRequestBuilder;
use states::StateRequestBuilder;

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
