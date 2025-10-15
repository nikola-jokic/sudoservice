use crate::service::{Service, ServiceError, Status};
use core::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use which::which;

const SYSTEMCTL: &str = "systemctl";
const SERVICE_FILE_PERMISSIONS: u32 = 0o644;

/// Configuration for a systemd service.
/// This struct holds all necessary information to create and manage a systemd service.
#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub username: Option<String>,
    pub executable: PathBuf,
    pub args: Option<Vec<String>>,
    pub working_directory: Option<PathBuf>,
    pub environment: Option<Vec<(String, String)>>,
    pub ch_root: Option<PathBuf>,
    pub restart: bool,
    pub restart_sec: Option<u32>,
}

impl fmt::Display for Config {
    fn fmt(&self, buf: &mut fmt::Formatter<'_>) -> fmt::Result {
        buf.write_str("[Unit]\n").unwrap();
        buf.write_fmt(format_args!("Description={}\n", self.display_name))
            .unwrap();

        buf.write_str("[Service]\n").unwrap();
        buf.write_str("StartLimitInterval=5\n").unwrap();
        buf.write_str("StartLimitBurst=10\n").unwrap();

        match self.args {
            Some(ref args) => {
                let args = args.join(" ");
                buf.write_fmt(format_args!(
                    "ExecStart={} {}\n",
                    self.executable.display(),
                    args
                ))
                .unwrap();
            }
            None => {
                buf.write_fmt(format_args!("ExecStart={}\n", self.executable.display()))
                    .unwrap();
            }
        }

        buf.write_str("KillSignal=SIGINT\n").unwrap();

        if self.restart {
            buf.write_str("Restart=always\n").unwrap();
        } else {
            buf.write_str("Restart=no\n").unwrap();
        }

        if self.ch_root.is_some() {
            buf.write_fmt(format_args!(
                "RootDirectory={}\n",
                self.ch_root.as_ref().unwrap().display()
            ))
            .unwrap();
        }

        if let Some(ref working_directory) = self.working_directory {
            buf.write_fmt(format_args!(
                "WorkingDirectory={}\n",
                working_directory.display()
            ))
            .unwrap();
        }

        if let Some(ref username) = self.username {
            buf.write_fmt(format_args!("User={}\n", username)).unwrap();
        }

        buf.write_str("RestartSec=120\n").unwrap();
        buf.write_str("EnvironmentFile=-/etc/sysconfig/\n").unwrap();

        if let Some(ref environment) = self.environment {
            for (key, value) in environment {
                buf.write_fmt(format_args!("Environment={}={}\n", key, value))
                    .unwrap();
            }
        }

        buf.write_str("[Install]\n").unwrap();
        buf.write_str("WantedBy=multi-user.target\n").unwrap();
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

impl Service for Systemd {
    fn install(&self) -> Result<(), ServiceError> {
        let dst = self.config_path();
        if dst.exists() {
            return Err(ServiceError {
                message: "Service file already exists".to_string(),
            });
        }

        let mut file = File::create(&dst).map_err(|err| ServiceError {
            message: format!("Failed to open service file: {:?}", err),
        })?;

        file.write_all(format!("{}", self.config).as_bytes())
            .map_err(|err| ServiceError {
                message: format!("Failed to write to service file: {:?}", err),
            })?;

        std::fs::set_permissions(
            &dst,
            std::fs::Permissions::from_mode(SERVICE_FILE_PERMISSIONS),
        )
        .map_err(|err| ServiceError {
            message: format!("Failed to set service file permissions: {:?}", err),
        })?;

        Command::new(SYSTEMCTL)
            .arg("enable")
            .arg(self.config.unit_name())
            .output()
            .map_err(|err| ServiceError {
                message: format!("Failed to enable {}: {:?}", self.config.unit_name(), err),
            })?;

        Command::new(SYSTEMCTL)
            .arg("daemon-reload")
            .output()
            .map_err(|err| ServiceError {
                message: format!("Failed to reload systemd daemon: {:?}", err),
            })?;

        Ok(())
    }

    fn uninstall(&self) -> Result<(), ServiceError> {
        Command::new(SYSTEMCTL)
            .arg("disable")
            .arg(self.config.unit_name())
            .output()
            .map_err(|err| ServiceError {
                message: format!("Failed to stop {}: {:?}", self.config.unit_name(), err),
            })?;

        fs::remove_file(self.config_path()).map_err(|_| ServiceError {
            message: "Failed to remove service file".to_string(),
        })?;

        Ok(())
    }

    fn start(&self) -> Result<(), ServiceError> {
        Command::new(SYSTEMCTL)
            .arg("start")
            .arg(self.config.unit_name())
            .output()
            .map_err(|err| ServiceError {
                message: format!("Failed to start {}: {:?}", self.config.unit_name(), err),
            })?;

        Ok(())
    }

    fn stop(&self) -> Result<(), ServiceError> {
        Command::new(SYSTEMCTL)
            .arg("stop")
            .arg(self.config.unit_name())
            .output()
            .map_err(|err| ServiceError {
                message: format!("Failed to stop {}: {:?}", self.config.unit_name(), err),
            })?;

        Ok(())
    }

    fn restart(&self) -> Result<(), ServiceError> {
        Command::new(SYSTEMCTL)
            .arg("restart")
            .arg(self.config.unit_name())
            .output()
            .map_err(|err| ServiceError {
                message: format!("Failed to restart {}: {:?}", self.config.unit_name(), err),
            })?;

        Ok(())
    }

    fn status(&self) -> Result<Status, ServiceError> {
        let output = Command::new(SYSTEMCTL)
            .arg("is-active")
            .arg(self.config.unit_name())
            .output()
            .map_err(|err| ServiceError {
                message: format!(
                    "Failed to get status of {}: {:?}",
                    self.config.unit_name(),
                    err
                ),
            })?;

        let status = String::from_utf8(output.stdout).unwrap();

        match status.trim() {
            "active" => Ok(Status::Running),
            "inactive" => {
                let output = Command::new(SYSTEMCTL)
                    .arg("list-unit-files")
                    .arg("-t")
                    .arg("service")
                    .arg(self.config.unit_name())
                    .output()
                    .map_err(|err| ServiceError {
                        message: format!(
                            "Failed to get status of {}: {:?}",
                            self.config.unit_name(),
                            err
                        ),
                    })?;

                if !output.status.success() {
                    return Err(ServiceError {
                        message: format!("Failed to get status of {}", self.config.unit_name()),
                    });
                }

                let status = String::from_utf8(output.stdout).unwrap();

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
