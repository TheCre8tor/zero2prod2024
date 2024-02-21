use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    startup::run(listener)?.await
}
