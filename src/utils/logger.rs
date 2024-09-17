use log::{error, info, warn};

pub fn init() {
    env_logger::init();
    info!("Logger initialized");
}
