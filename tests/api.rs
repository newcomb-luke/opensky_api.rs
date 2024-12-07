use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use dotenv::dotenv;
use opensky_api::{tracks::FlightTrack, Flight, OpenSkyApi, States};

#[tokio::test]
async fn get_all_states() {
    let opensky_api = OpenSkyApi::new();

    let states_request = opensky_api.get_states();

    let _states = states_request.send().await.unwrap();
}

#[tokio::test]
async fn get_states_at_time() {
    dotenv().ok();

    let username = env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
    let password = env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");

    let opensky_api = OpenSkyApi::with_login(username, password);

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let states_request = opensky_api.get_states().at_time(now.as_secs());

    let _states = states_request.send().await.unwrap();
}

#[tokio::test]
async fn serde_states() {
    dotenv().ok();

    let username = env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
    let password = env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");

    let opensky_api = OpenSkyApi::with_login(username, password);

    let states_request = opensky_api
        .get_states()
        .at_time(1458564121)
        .with_icao24("3c6444".to_string());

    let states = states_request.send().await.unwrap();

    let json = serde_json::to_string(&states).unwrap();
    assert_eq!(
        json,
        r#"{"time":1458564121,"states":[{"icao24":"3c6444","callsign":"DLH9LF  ","origin_country":"Germany","time_position":1458564120,"last_contact":1458564120,"longitude":6.1546,"latitude":50.1964,"baro_altitude":9639.3,"on_ground":false,"velocity":232.88,"true_track":98.26,"vertical_rate":4.55,"sensors":null,"geo_altitude":9547.86,"squawk":"1000","spi":false,"position_source":"ADSB","category":null}]}"#,
    );

    let states: States = serde_json::from_str(&json).unwrap();
    println!("states: {:#?}", states);
}

#[tokio::test]
async fn get_all_flights() {
    dotenv().ok();
    let username = env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
    let password = env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");

    let opensky_api = OpenSkyApi::with_login(username, password);

    let begin = 1517227200;
    let end = 1517230800;

    let flights_request = opensky_api.get_flights(begin, end);

    let _flights = flights_request.send().await.unwrap();
}

#[tokio::test]
async fn serde_flights() {
    dotenv().ok();
    let username = env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
    let password = env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");

    let opensky_api = OpenSkyApi::with_login(username, password);

    let begin = 1517230000;
    let end = 1517230800;

    let mut flights_request = opensky_api.get_flights(begin, end);
    flights_request.by_departure("RCTP".to_string());

    let flights = flights_request.send().await.unwrap();

    let json = serde_json::to_string(&flights).unwrap();
    assert_eq!(
        json,
        r#"[{"icao24":"70e050","first_seen":1517230718,"est_departure_airport":"RCTP","last_seen":1517234349,"est_arrival_airport":null,"callsign":"JCC667  ","est_departure_airport_horiz_distance":2254,"est_departure_airport_vert_distance":577,"est_arrival_airport_horiz_distance":null,"est_arrival_airport_vert_distance":null,"departure_airport_candidates_count":1,"arrival_airport_candidates_count":0},{"icao24":"89906d","first_seen":1517230549,"est_departure_airport":"RCTP","last_seen":1517236873,"est_arrival_airport":"RPLL","callsign":"EVA261  ","est_departure_airport_horiz_distance":3571,"est_departure_airport_vert_distance":478,"est_arrival_airport_horiz_distance":7249,"est_arrival_airport_vert_distance":243,"departure_airport_candidates_count":1,"arrival_airport_candidates_count":0}]"#,
    );

    let flights: Vec<Flight> = serde_json::from_str(&json).unwrap();
    println!("flights: {:#?}", flights);
}

#[tokio::test]
async fn get_tracks() {
    dotenv().ok();
    let username = env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
    let password = env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");

    let opensky_api = OpenSkyApi::with_login(username, password);

    let track_request = opensky_api.get_tracks("8990ed".to_string());

    let track = track_request.send().await.unwrap();
    let json = serde_json::to_string(&track).unwrap();
    let track: FlightTrack = serde_json::from_str(&json).unwrap();
    println!("tracks: {:#?}", track);
}
