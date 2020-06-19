# Windows Service Template Project

Template for installing a windows service.

It registers a logger with the `winlog` crate to log messages to the Windows Event Log.

## How to get started

Make sure you have Rust and InnoSetup Compiler installed. See Installer section below

Clone the repository to a folder

Change the name of the root folder to match your service name

Change the `SERVICE_NAME` constant in the following files:

- src/bin/install.rs
- src/bin/uninstall.rs
- src/main.rs
- Cargo.toml

Take special note of the variables in `install.rs` and change them accordingly:

```rust
const SERVICE_NAME: &str = "MySchedulerService";
const SERVICE_DISPLAY_NAME: &str = "My scheduler service";
const SERVICE_START_TYPE: ServiceStartType = ServiceStartType::OnDemand;
```

Change the paths in `setup/installer.iss` to match your project path and build binaries, you should
only need to edit the top level variables, including the `Profile` variable to match your build profile.

Run `cargo build`

Open `installer.iss` in InnoSetup and compile the installer

Get the installer from the output dir specified in `installer.iss`

Run the installer on the system you want to install the service on

Uninstall using Add/Remove programs if needed

### Some tips

The service will not actually get uninstalled if you keep the service manager window open. Close it
and reopen for it to be removed (or don't keep it open when you uninstall the service)

Installing while having Event Viewer open can cause problems since we're registering our log
when installing.

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