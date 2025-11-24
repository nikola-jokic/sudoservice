use crate::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use which::which;

#[macro_use]
mod macros;
mod install;
mod service;
mod unit;

pub use install::Install;
pub use service::Service;
pub use unit::Unit;

const SYSTEMCTL: &str = "systemctl";
const SERVICE_FILE_PERMISSIONS: u32 = 0o644;

type Result<T> = std::result::Result<T, Error>;

/// Configuration for a systemd service.
/// This struct holds all necessary information to create and manage a systemd service.
#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub unit: Unit,
    pub service: Service,
    pub install: Install,
}

impl fmt::Display for Config {
    fn fmt(&self, buf: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.unit.fmt(buf)?;
        self.service.fmt(buf)?;
        self.install.fmt(buf)?;
        Ok(())
    }
}

impl Config {
    fn unit_name(&self) -> String {
        format!("{}.service", self.name)
    }
}

/// A service manager for systemd.
/// This struct provides methods to install, uninstall, start, stop, restart, and check the status of a systemd service.
#[derive(Debug)]
pub struct Systemd {
    config: Config,
}

impl Systemd {
    /// Checks if systemd is available on the system.
    /// This is done by checking if the `systemctl` command is present in the system's PATH.
    pub fn is_available() -> bool {
        which(SYSTEMCTL).is_ok()
    }
}

impl Systemd {
    /// Creates a new `Systemd` service manager with the given configuration.
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    fn config_path(&self) -> PathBuf {
        PathBuf::from(format!("/etc/systemd/system/{}", self.config.unit_name()))
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Running => write!(f, "Running"),
            Status::Stopped => write!(f, "Stopped"),
            Status::Unknown => write!(f, "Unknown"),
            Status::NotInstalled => write!(f, "Not installed"),
            Status::Failed => write!(f, "Failed"),
        }
    }
}

impl Systemd {
    pub fn install(&self) -> Result<()> {
        let dst = self.config_path();
        if dst.exists() {
            return Err(Error::ValidationError(format!(
                "Service file '{dst:?}' already exists"
            )));
        }
        let mut file = File::create(&dst)?;
        file.write_all(format!("{}", self.config).as_bytes())?;

        std::fs::set_permissions(
            &dst,
            std::fs::Permissions::from_mode(SERVICE_FILE_PERMISSIONS),
        )?;

        match Command::new(SYSTEMCTL)
            .arg("enable")
            .arg(self.config.unit_name())
            .output()?
        {
            output if output.status.success() => {}
            output => {
                return Err(Error::CommandError(format!(
                    "Failed to enable service: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
        }

        match Command::new(SYSTEMCTL).arg("daemon-reload").output()? {
            output if output.status.success() => {}
            output => {
                return Err(Error::CommandError(format!(
                    "Failed to reload systemd daemon: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
        }

        Ok(())
    }

    pub fn uninstall(&self) -> Result<()> {
        match Command::new(SYSTEMCTL)
            .arg("disable")
            .arg(self.config.unit_name())
            .output()?
        {
            output if output.status.success() => {}
            output => {
                return Err(Error::CommandError(format!(
                    "Failed to disable service: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
        }

        fs::remove_file(self.config_path())?;

        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        match Command::new(SYSTEMCTL)
            .arg("start")
            .arg(self.config.unit_name())
            .output()?
        {
            output if output.status.success() => {}
            output => {
                return Err(Error::CommandError(format!(
                    "Failed to start service: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
        }

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        match Command::new(SYSTEMCTL)
            .arg("stop")
            .arg(self.config.unit_name())
            .output()?
        {
            output if output.status.success() => {}
            output => {
                return Err(Error::CommandError(format!(
                    "Failed to stop service: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
        }

        Ok(())
    }

    pub fn restart(&self) -> Result<()> {
        match Command::new(SYSTEMCTL)
            .arg("restart")
            .arg(self.config.unit_name())
            .output()?
        {
            output if output.status.success() => {}
            output => {
                return Err(Error::CommandError(format!(
                    "Failed to restart service: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
        }

        Ok(())
    }

    pub fn status(&self) -> Result<Status> {
        let status = match Command::new(SYSTEMCTL)
            .arg("is-active")
            .arg(self.config.unit_name())
            .output()?
        {
            output if output.status.success() => {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            }
            output => {
                return Err(Error::CommandError(format!(
                    "Failed to get service status: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
        };

        match status.as_str() {
            "active" => Ok(Status::Running),
            "inactive" => {
                let status = match Command::new(SYSTEMCTL)
                    .arg("list-unit-files")
                    .arg("-t")
                    .arg("service")
                    .arg(self.config.unit_name())
                    .output()?
                {
                    output if output.status.success() => {
                        String::from_utf8_lossy(&output.stdout).to_string()
                    }
                    output => {
                        return Err(Error::CommandError(format!(
                            "Failed to list unit files: {}",
                            String::from_utf8_lossy(&output.stderr)
                        )));
                    }
                };

                if status.contains(&self.config.unit_name()) {
                    Ok(Status::Stopped)
                } else {
                    Ok(Status::NotInstalled)
                }
            }
            "activating" => Ok(Status::Running),
            "failed" => Ok(Status::Failed),
            _ => Ok(Status::NotInstalled),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Status {
    Running,
    Stopped,
    Unknown,
    NotInstalled,
    Failed,
}
