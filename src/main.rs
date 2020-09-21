#[macro_use]
extern crate lazy_static;
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

mod settings;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

#[tokio::main]
async fn main() {
    let cfg_route = warp::path("local")
        .and(with_cfg(CONFIG.clone()))
        .and_then(cfg_handler);

    let global_cfg_route = warp::path("global").and_then(global_cfg_handler);

    println!(
        "Server started at localhost:{} and ENV: {}",
        CONFIG.server.port, CONFIG.env
    );

    warp::serve(cfg_route.or(global_cfg_route))
        .run(([0, 0, 0, 0], CONFIG.server.port))
        .await;
}

async fn cfg_handler(cfg: settings::Settings) -> Result<impl Reply, Rejection> {
    Ok(format!(
        "Running on port: {} with url: {}",
        cfg.server.port, cfg.server.url
    ))
}

async fn global_cfg_handler() -> Result<impl Reply, Rejection> {
    Ok(format!(
        "Running with interval: {} and rules: {:?}",
        CONFIG.job.interval, CONFIG.rules
    ))
}

fn with_cfg(
    cfg: settings::Settings,
) -> impl Filter<Extract = (settings::Settings,), Error = Infallible> + Clone {
    warp::any().map(move || cfg.clone())
}
