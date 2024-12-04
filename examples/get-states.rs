use chrono::{Local, SecondsFormat};
use colored::Colorize;
use log::{error, info, warn, LevelFilter};
use std::{env, io::Write};

use opensky_api::OpenSkyApi;
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

    let states_request = opensky_api
        .get_states()
        .at_time(1458564121)
        .with_icao24("3c6444".to_string());

    let result = states_request.send().await;
    match result {
        Ok(states) => {
            info!("get result: {:#?}", states.states);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
}
