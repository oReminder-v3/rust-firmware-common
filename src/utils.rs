use crate::commandline::exit_with_cause;

pub fn get_email() -> String {
    std::env::var("OREMINDER_EMAIL").unwrap_or_else(|_| {
        exit_with_cause("Environment variable OREMINDER_EMAIL is not set!");
        String::new()
    })
}

pub fn get_password() -> String {
    std::env::var("OREMINDER_PASSWORD").unwrap_or_else(|_| {
        exit_with_cause("Environment variable OREMINDER_PASSWORD is not set!");
        String::new()
    })
}
