use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ServiceError {
    pub message: String,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Service error: {}", self.message)
    }
}

impl Error for ServiceError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub trait Service {
    fn install(&self) -> Result<(), ServiceError>;
    fn uninstall(&self) -> Result<(), ServiceError>;
    fn start(&self) -> Result<(), ServiceError>;
    fn stop(&self) -> Result<(), ServiceError>;
    fn restart(&self) -> Result<(), ServiceError>;
    fn status(&self) -> Result<Status, ServiceError>;
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Status {
    Running,
    Stopped,
    Unknown,
    NotInstalled,
    Failed,
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

pub enum Type {
    Systemd,
}
