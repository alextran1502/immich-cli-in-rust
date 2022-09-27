use openapi::{
    apis::{authentication_api, configuration::Configuration},
    models::{LoginCredentialDto, LoginResponseDto},
};
use simplelog::*;
use std::error::Error;

pub async fn login(
    email: &str,
    password: &str,
    api_config: &Configuration,
) -> Result<LoginResponseDto, Box<dyn Error>> {
    info!("Logging in...");

    let login_payload: LoginCredentialDto = LoginCredentialDto {
        email: email.to_string(),
        password: password.to_string(),
    };

    let auth_user = match authentication_api::login(&api_config, login_payload).await {
        Ok(auth_user) => {
            info!("Logged in as {}", auth_user.user_email);
            auth_user
        }
        Err(_) => {
            error!("Failed to sign in");
            return Err("Failed to sign in".into());
        }
    };

    Ok(auth_user)
}
