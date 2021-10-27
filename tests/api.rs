use opensky_api::OpenSkyApi;

#[tokio::test]
async fn get_all_states() {
    let opensky_api = OpenSkyApi::new();

    let states_request = opensky_api.get_states();

    let _states = states_request.send().await.unwrap();
}

#[tokio::test]
async fn get_states_at_time() {
    let opensky_api = OpenSkyApi::new();

    let states_request = opensky_api.get_states().at_time(1635347380);

    let _states = states_request.send().await.unwrap();
}

#[tokio::test]
async fn get_all_flights() {
    let opensky_api = OpenSkyApi::new();

    let flights_request = opensky_api.get_flights(1635314980, 1635347380);

    let _flights = flights_request.send().await.unwrap();
}
