use std::error::Error;

pub fn sign_in(email: &str, password: &str) -> Result<(), Box<dyn Error>> {
    println!("signing in... email: {}, password: {}", email, password);

    return Ok(());
}

// pub fn scan_directory(directory: &str) -> Result<(), Box<dyn Error>> {
//     println!("scanning directory... {}", directory);

//     return Ok(());
// }
