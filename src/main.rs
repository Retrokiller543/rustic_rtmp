// This file will contain the main function that starts the server. It should be responsible for setting up the server and starting the main event loop.
mod server;

use crate::server::server::Server;
use flexi_logger::{FileSpec, Logger, WriteMode};
use log::info;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _logger = Logger::try_with_str("info, my::critical::module=trace")?
    .log_to_file(FileSpec::default())
    .write_mode(WriteMode::BufferAndFlush)
    .start()?;
    let server = Server::new("0.0.0.0:1935".to_owned());
    info!("Starting server");
    server.run().await?;
    Ok(())
}
