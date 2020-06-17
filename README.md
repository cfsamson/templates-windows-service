# Windows Service Template Project

Template for installing a windows service.

It registers a logger with the `winlog` crate to log messages to the Windows Event Log.

## How to get started

Change the `SERVICE_NAME` constant in the following files:

- src/bin/install.rs
- src/bin/uninstall.rs
- src/main.rs
- Cargo.toml

## Development

The main entry point of the service is `app::app()`. Put the program logic entry point here. See
https://github.com/mullvad/windows-service-rs/blob/master/examples/ping_service.rs for an example
service.

## Installer

To get the installer to work you need InnoSetup, get it here: https://jrsoftware.org/isdl.php#qsp

Open the Compiler and locate the file located in `setup/installer.iss`

Change the variables and paths accordingly (the example assumes a project path `C:\dev\rust\scheduler-service\`)

Compile the installer.
