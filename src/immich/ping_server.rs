use openapi::apis::configuration::Configuration;
use simplelog::*;

pub async fn ping_server(api_config: &Configuration) {
    info!("Pinging server...");
    match openapi::apis::server_info_api::ping_server(&api_config).await {
        Ok(_) => {}
        Err(_) => {
            error!("[ERROR] Failed to connect to server - check URL");
            std::process::exit(1);
        }
    }
}