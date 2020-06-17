# Windows Service Template Project

Template for installing a windows service.

It registers a logger with the `winlog` crate to log messages to the Windows Event Log.

## How to get started

Change the `SERVICE_NAME` constant in the following files:

- src/bin/install.rs
- src/bin/uninstall.rs
- src/main.rs
- Cargo.toml

Take special note of the variables in `install.rs` and change them accordingly:

```rust
const SERVICE_NAME: &str = "MySchedulerService";
const SERVICE_BINARY_PATH: &str = r"C:\dev\scheduler-service\target\debug\scheduler_service.exe";
const SERVICE_DISPLAY_NAME: &str = "My scheduler service";
const SERVICE_START_TYPE: ServiceStartType = ServiceStartType::OnDemand;
```

## Development

The main entry point of the service is `app::app()`. Put the program logic entry point here. See
https://github.com/mullvad/windows-service-rs/blob/master/examples/ping_service.rs for an example
service.

## Installer

To get the installer to work you need InnoSetup, get it here: https://jrsoftware.org/isdl.php#qsp

Open the Compiler and locate the file located in `setup/installer.iss`

Change the variables and paths accordingly (the example assumes a project path `C:\dev\rust\scheduler-service\`)

Compile the installer.

## Additional notes

See https://github.com/mullvad/windows-service-rs/issues/43 for the reason we build `windows-service`
from the github repo's master branch and not using the published crate.