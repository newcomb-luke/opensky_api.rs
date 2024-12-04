use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use opensky_api::OpenSkyApi;

#[tokio::test]
async fn get_all_states() {
    let opensky_api = OpenSkyApi::new();

    let states_request = opensky_api.get_states();

    let _states = states_request.send().await.unwrap();
}

#[tokio::test]
async fn get_states_at_time() {
    dotenv::dotenv().ok();

    let username = env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
    let password = env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");

    let opensky_api = OpenSkyApi::with_login(username, password);

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let states_request = opensky_api.get_states().at_time(now.as_secs());

    let _states = states_request.send().await.unwrap();
}

/*
#[tokio::test]
async fn get_all_flights() {
    let username = env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
    let password = env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");

    let opensky_api = OpenSkyApi::with_login(username, password);

    let begin = 1517227200;
    let end = 1517230800;

    let flights_request = opensky_api.get_flights(begin, end);

    let _flights = flights_request.send().await.unwrap();
}
*/
