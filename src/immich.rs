use std::error::Error;

use openapi::{
    apis::{authentication_api, configuration::Configuration},
    models::{LoginCredentialDto, LoginResponseDto},
};

pub async fn sign_in(
    email: &str,
    password: &str,
    api_config: &Configuration,
) -> Result<LoginResponseDto, Box<dyn Error>> {
    let login_payload: LoginCredentialDto = LoginCredentialDto {
        email: email.to_string(),
        password: password.to_string(),
    };

    let auth_user = authentication_api::login(&api_config, login_payload).await?;

    Ok(auth_user)
}

// pub fn scan_directory(directory: &str) -> Result<(), Box<dyn Error>> {
//     println!("scanning directory... {}", directory);

//     return Ok(());
// }
