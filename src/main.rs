use std::net::TcpListener;

use exporter::configuration::get_configuration;
use exporter::startup::run;
use exporter::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_subscriber(get_subscriber("exporter".to_string(), "info".to_string()));
    let settings = get_configuration().expect("Failed to read configuration.");

    let address = format!("{}:{}", settings.http.host, settings.http.port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");


    run(listener)?.await
}
