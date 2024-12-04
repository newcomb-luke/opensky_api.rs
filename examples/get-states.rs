use std::env;

use opensky_api::OpenSkyApi;
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let username=env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
    let password=env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");

    let opensky_api = OpenSkyApi::with_login(username, password);

    let states_request = opensky_api.get_states().with_icao24("7806e9".to_string());

    let result = states_request.send().await;
    match result {
        Ok(states) => {
            println!("{:#?}", states);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}