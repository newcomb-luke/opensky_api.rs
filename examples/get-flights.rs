use chrono::{Local, SecondsFormat};
use colored::Colorize;
use log::{error, info, LevelFilter};
use std::{
    env,
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use opensky_network::OpenSkyApi;
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    env_logger::Builder::new()
        .format(|buf, record| {
            let time = Local::now()
                .to_rfc3339_opts(SecondsFormat::Millis, true)
                .as_str()
                .bright_blue();
            let level = record.level().as_str();
            let colored_level = match record.level().to_level_filter() {
                LevelFilter::Info => level.green(),
                LevelFilter::Warn => level.yellow(),
                LevelFilter::Error => level.red(),
                _ => level.into(),
            };
            writeln!(buf, "{} [{}] - {}", time, colored_level, record.args(),)
        })
        .filter(None, LevelFilter::Debug)
        .init();

    let username = env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
    let password = env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");

    let opensky_api = OpenSkyApi::with_login(username, password);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    info!("now: {}", now);
    let mut flights_request = opensky_api.get_flights(now - 7 * 24 * 60 * 60, now);
    flights_request.by_aircraft("8990ed".to_string());

    let result = flights_request.send().await;
    match result {
        Ok(flights) => {
            info!("get result: {:#?}", flights);
            info!("Get {} flights", flights.len());
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
}
