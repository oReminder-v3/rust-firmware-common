use std::process::exit;

pub fn exit_with_cause(message: &str) {
    error!("{}", message);
    exit(1);
}
