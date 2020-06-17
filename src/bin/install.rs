use windows_service::service::ServiceStartType;
use once_cell::unsync::Lazy;
use std::{thread, time::Duration};

const SERVICE_NAME: &str = "MySchedulerService";
const SERVICE_DISPLAY_NAME: &str = "My scheduler service";
const SERVICE_START_TYPE: ServiceStartType = ServiceStartType::OnDemand;

// Points to the path where InnoSetup places our binary files, if this doesn't work either
// fill in the value manually or look up the env_var for Program Files(x86) on the version of
// windows you're running on
thread_local!{
    static SERVICE_BINARY_PATH: Lazy<String> = Lazy::new(|| {
        let program_files = std::env::var("PROGRAMFILES(X86)").expect("Can't find 'Program Files(x86)' folder");
        let exe_name = env!("CARGO_PKG_NAME");
        format!(r"{}\{}\{}.exe", program_files, SERVICE_NAME, exe_name)
    });
}

#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    use std::ffi::OsString;
    use windows_service::{
        service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceType},
        service_manager::{ServiceManager, ServiceManagerAccess},
    };

    // Register the windows logger
    winlog::try_register(SERVICE_NAME).unwrap();

    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    // The path to the service binary to install
    let sbp = SERVICE_BINARY_PATH.with(|sbp| (*sbp).clone());
    let service_binary_path = std::path::PathBuf::from(sbp);

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
    let service = service_manager.create_service(&service_info, ServiceAccess::START)?;
    // Start the service once installed
    service.start(&[""])?;
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}

#[test]
fn exe_dir() {
    let sbp = SERVICE_BINARY_PATH.with(|sbp| (*sbp).clone());
    println!("{}", sbp);
}
