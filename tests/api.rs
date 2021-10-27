use opensky_api::OpenSkyApi;

#[tokio::test]
async fn get_all_states() {
    let opensky_api = OpenSkyApi::new();

    let states_request = opensky_api.get_states();

    let _states = states_request.send().await.unwrap();
}

#[tokio::test]
async fn get_states_at_time() {
    todo!();
}
