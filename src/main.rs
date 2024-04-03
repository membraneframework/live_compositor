use log::info;

mod state;
mod config;
mod error;
mod logger;
mod routes;
mod server;
mod types;

#[cfg(test)]
mod snapshot_tests;

fn main() {
    #[cfg(feature = "web_renderer")]
    {
        use compositor_chromium::cef::bundle_for_development;

        let target_path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_owned();
        if bundle_for_development(&target_path).is_err() {
            panic!("Build process helper first. For release profile use: cargo build -r --bin process_helper");
        }
    }

    ffmpeg_next::format::network::init();

    server::run();

    info!("Received exit signal. Terminating...")
    // TODO: add graceful shutdown
}
