use colored::Colorize;
use log::{error, info, LevelFilter};
use opensky_network::OpenSkyApi;
use std::{env, io::Write};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    env_logger::Builder::new()
        .format(|buf, record| {
            let time = chrono::Local::now()
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                .as_str()
                .bright_blue();
            let level = record.level().as_str();
            let colored_level = match record.level().to_level_filter() {
                log::LevelFilter::Info => level.green(),
                log::LevelFilter::Warn => level.yellow(),
                log::LevelFilter::Error => level.red(),
                _ => level.into(),
            };
            writeln!(buf, "{} [{}] - {}", time, colored_level, record.args(),)
        })
        .filter(None, LevelFilter::Debug)
        .init();

    let username = env::var("OPENSKY_USER").expect("OPENSKY_USER environment variable not set");
    let password = env::var("OPENSKY_PASS").expect("OPENSKY_PASS environment variable not set");

    let opensky_api = OpenSkyApi::with_login(username, password);

    let track_request = opensky_api.get_tracks("8990ed".to_string());

    let result = track_request.send().await;

    match result {
        Ok(tracks) => {
            info!("get result: {:#?}", tracks);
            let json = serde_json::to_string(&tracks).unwrap();
            info!("get result json: {}", json);
            info!("Get {} waypoints", tracks.path.len());
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
}
