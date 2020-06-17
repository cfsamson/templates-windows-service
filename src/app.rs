use crate::SERVICE_NAME;
use log::*;

pub fn app() {
    winlog::init(SERVICE_NAME).unwrap();
    info!("SERVICE STARTED");
}