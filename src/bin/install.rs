use windows_service::service::{ServiceStartType};

const SERVICE_NAME: &str = "MySchedulerService";
const SERVICE_BINARY_PATH: &str = r"C:\Users\cf\dev\rust\samson\samson-log-cleaner\target\debug\samson_scheduler.exe";
const SERVICE_DISPLAY_NAME: &str = "Samson Scheduler";
const SERVICE_START_TYPE: ServiceStartType = ServiceStartType::OnDemand;

#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    use std::ffi::OsString;
    use windows_service::{
        service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
        service_manager::{ServiceManager, ServiceManagerAccess},
    };

    // Register the windows logger
    winlog::try_register(SERVICE_NAME).unwrap();

    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    // The path to the service binary to install
    let service_binary_path = std::path::PathBuf::from(SERVICE_BINARY_PATH);

    let service_info = ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from(SERVICE_DISPLAY_NAME),
        service_type: ServiceType::OWN_PROCESS,
        start_type: SERVICE_START_TYPE,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None, // run as System
        account_password: None,
    };
    let _service = service_manager.create_service(&service_info, ServiceAccess::empty())?;
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}