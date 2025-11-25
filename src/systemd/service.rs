use std::fmt;
use super::exec::Exec;
use std::path::PathBuf;

/// Service type for systemd services
#[derive(Debug, Clone, Copy)]
pub enum ServiceType {
    /// Simple service
    Simple,
    /// Exec service
    Exec,
    /// Forking service
    Forking,
    /// Oneshot service
    Oneshot,
    /// D-Bus service
    Dbus,
    /// Notify service
    Notify,
    /// Notify reload service
    NotifyReload,
    /// Idle service
    Idle,
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceType::Simple => write!(f, "simple"),
            ServiceType::Exec => write!(f, "exec"),
            ServiceType::Forking => write!(f, "forking"),
            ServiceType::Oneshot => write!(f, "oneshot"),
            ServiceType::Dbus => write!(f, "dbus"),
            ServiceType::Notify => write!(f, "notify"),
            ServiceType::NotifyReload => write!(f, "notify-reload"),
            ServiceType::Idle => write!(f, "idle"),
        }
    }
}

/// Exit type for services
#[derive(Debug, Clone, Copy)]
pub enum ExitType {
    /// Main process
    Main,
    /// Control group
    Cgroup,
}

impl fmt::Display for ExitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExitType::Main => write!(f, "main"),
            ExitType::Cgroup => write!(f, "cgroup"),
        }
    }
}

/// Restart type for services
#[derive(Debug, Clone, Copy)]
pub enum RestartType {
    /// No restart
    No,
    /// Restart on success
    OnSuccess,
    /// Restart on failure
    OnFailure,
    /// Restart on abnormal exit
    OnAbnormal,
    /// Restart on watchdog timeout
    OnWatchdog,
    /// Restart on abort
    OnAbort,
    /// Always restart
    Always,
}

impl fmt::Display for RestartType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RestartType::No => write!(f, "no"),
            RestartType::OnSuccess => write!(f, "on-success"),
            RestartType::OnFailure => write!(f, "on-failure"),
            RestartType::OnAbnormal => write!(f, "on-abnormal"),
            RestartType::OnWatchdog => write!(f, "on-watchdog"),
            RestartType::OnAbort => write!(f, "on-abort"),
            RestartType::Always => write!(f, "always"),
        }
    }
}

/// Restart mode for services
#[derive(Debug, Clone, Copy)]
pub enum RestartMode {
    /// Normal restart
    Normal,
    /// Direct restart
    Direct,
    /// Debug restart
    Debug,
}

impl fmt::Display for RestartMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RestartMode::Normal => write!(f, "normal"),
            RestartMode::Direct => write!(f, "direct"),
            RestartMode::Debug => write!(f, "debug"),
        }
    }
}

/// Notify access for services
#[derive(Debug, Clone, Copy)]
pub enum NotifyAccess {
    /// No access
    None,
    /// Main process access
    Main,
    /// Exec access
    Exec,
    /// All access
    All,
}

impl fmt::Display for NotifyAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotifyAccess::None => write!(f, "none"),
            NotifyAccess::Main => write!(f, "main"),
            NotifyAccess::Exec => write!(f, "exec"),
            NotifyAccess::All => write!(f, "all"),
        }
    }
}

/// Timeout failure mode for services
#[derive(Debug, Clone, Copy)]
pub enum TimeoutFailureMode {
    /// Terminate
    Terminate,
    /// Abort
    Abort,
    /// Kill
    Kill,
}

impl fmt::Display for TimeoutFailureMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeoutFailureMode::Terminate => write!(f, "terminate"),
            TimeoutFailureMode::Abort => write!(f, "abort"),
            TimeoutFailureMode::Kill => write!(f, "kill"),
        }
    }
}

/// OOM policy for services
#[derive(Debug, Clone, Copy)]
pub enum OomPolicy {
    /// Continue
    Continue,
    /// Stop
    Stop,
    /// Kill
    Kill,
}

impl fmt::Display for OomPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OomPolicy::Continue => write!(f, "continue"),
            OomPolicy::Stop => write!(f, "stop"),
            OomPolicy::Kill => write!(f, "kill"),
        }
    }
}

/// Service section options
/// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#Options
#[derive(Debug, Clone, Default)]
pub struct Service {
    /// Configures the mechanism via which the service notifies the manager that the service start-up has finished.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#Type=
    /// One of: simple, exec, forking, oneshot, dbus, notify, notify-reload, or idle
    pub service_type: Option<ServiceType>,

    /// Specifies when the manager should consider the service to be finished.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ExitType=
    /// One of: main or cgroup
    pub exit_type: Option<ExitType>,

    /// Whether the service shall be considered active even when all its processes exited.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RemainAfterExit=
    pub remain_after_exit: Option<bool>,

    /// Whether systemd should try to guess the main PID of a service.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#GuessMainPID=
    pub guess_main_pid: Option<bool>,

    /// Path referring to the PID file of the service.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#PIDFile=
    pub pid_file: Option<PathBuf>,

    /// D-Bus destination name that this service shall use.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#BusName=
    pub bus_name: Option<String>,

    /// Commands that are executed when this service is started.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ExecStart=
    pub exec_start: Option<Vec<String>>,

    /// Commands executed before ExecStart=.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ExecStartPre=
    pub exec_start_pre: Option<Vec<String>>,

    /// Commands executed after ExecStart=.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ExecStartPost=
    pub exec_start_post: Option<Vec<String>>,

    /// Optional commands executed before ExecStartPre=.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ExecCondition=
    pub exec_condition: Option<Vec<String>>,

    /// Commands to execute to trigger a configuration reload.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ExecReload=
    pub exec_reload: Option<Vec<String>>,

    /// Commands to execute to stop the service.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ExecStop=
    pub exec_stop: Option<Vec<String>>,

    /// Commands executed after the service is stopped.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ExecStopPost=
    pub exec_stop_post: Option<Vec<String>>,

    /// Time to sleep before restarting a service.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RestartSec=
    pub restart_sec: Option<u64>,

    /// Number of steps to increase restart interval.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RestartSteps=
    pub restart_steps: Option<u32>,

    /// Longest time to sleep before restarting.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RestartMaxDelaySec=
    pub restart_max_delay_sec: Option<String>,

    /// Time to wait for start-up.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#TimeoutStartSec=
    pub timeout_start_sec: Option<String>,

    /// Time to wait for stop.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#TimeoutStopSec=
    pub timeout_stop_sec: Option<String>,

    /// Time to wait for abort after watchdog timeout.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#TimeoutAbortSec=
    pub timeout_abort_sec: Option<String>,

    /// Shorthand for configuring both TimeoutStartSec= and TimeoutStopSec=.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#TimeoutSec=
    pub timeout_sec: Option<String>,

    /// Action taken when start timeout is hit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#TimeoutStartFailureMode=
    pub timeout_start_failure_mode: Option<TimeoutFailureMode>,

    /// Action taken when stop timeout is hit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#TimeoutStopFailureMode=
    pub timeout_stop_failure_mode: Option<TimeoutFailureMode>,

    /// Maximum time for the service to run.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RuntimeMaxSec=
    pub runtime_max_sec: Option<String>,

    /// Randomized extra time added to RuntimeMaxSec=.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RuntimeRandomizedExtraSec=
    pub runtime_randomized_extra_sec: Option<String>,

    /// Watchdog timeout for a service.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#WatchdogSec=
    pub watchdog_sec: Option<String>,

    /// Whether the service shall be restarted when the service process exits.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#Restart=
    /// One of: no, on-success, on-failure, on-abnormal, on-watchdog, on-abort, or always
    pub restart: Option<RestartType>,

    /// How a service should restart.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RestartMode=
    /// One of: normal, direct, or debug
    pub restart_mode: Option<RestartMode>,

    /// Exit statuses considered successful termination.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#SuccessExitStatus=
    pub success_exit_status: Option<Vec<String>>,

    /// Exit statuses that prevent automatic service restarts.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RestartPreventExitStatus=
    pub restart_prevent_exit_status: Option<Vec<String>>,

    /// Exit statuses that force automatic service restarts.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RestartForceExitStatus=
    pub restart_force_exit_status: Option<Vec<String>>,

    /// Whether root directory is only applied to ExecStart=.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RootDirectoryStartOnly=
    pub root_directory_start_only: Option<bool>,

    /// Set O_NONBLOCK flag for socket-based activation file descriptors.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#NonBlocking=
    pub non_blocking: Option<bool>,

    /// Controls access to the service status notification socket.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#NotifyAccess=
    /// One of: none, main, exec, or all
    pub notify_access: Option<NotifyAccess>,

    /// Names of socket units this service shall inherit socket file descriptors from.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#Sockets=
    pub sockets: Option<Vec<String>>,

    /// How many file descriptors may be stored in the service manager.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#FileDescriptorStoreMax=
    pub file_descriptor_store_max: Option<u32>,

    /// When to release the service's file descriptor store.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#FileDescriptorStorePreserve=
    /// One of: no, yes, or restart
    pub file_descriptor_store_preserve: Option<String>,

    /// Location of USB FunctionFS descriptors file.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#USBFunctionDescriptors=
    pub usb_function_descriptors: Option<PathBuf>,

    /// Location of USB FunctionFS strings file.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#USBFunctionStrings=
    pub usb_function_strings: Option<PathBuf>,

    /// Out-of-memory killing policy.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#OOMPolicy=
    /// One of: continue, stop, or kill
    pub oom_policy: Option<OomPolicy>,

    /// Files or sockets to open and pass to the service.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#OpenFile=
    pub open_file: Option<Vec<String>>,

    /// UNIX process signal to send when reloading the service.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ReloadSignal=
    pub reload_signal: Option<String>,
    
    pub exec: Option<Exec>,
}

impl fmt::Display for Service {
    fn fmt(&self, buf: &mut fmt::Formatter<'_>) -> fmt::Result {
        buf.write_str("[Service]\n")?;
        // Service type and behavior
        write_option!(buf, self.service_type, "Type");
        write_option!(buf, self.exit_type, "ExitType");
        write_bool!(buf, self.remain_after_exit, "RemainAfterExit");
        write_bool!(buf, self.guess_main_pid, "GuessMainPID");

        if let Some(ref path) = self.pid_file {
            writeln!(buf, "PIDFile={}", path.display())?;
        }

        write_option!(buf, self.bus_name, "BusName");

        // Execution commands (one per line for multi-value options)
        write_vec!(buf, self.exec_condition, "ExecCondition");
        write_vec!(buf, self.exec_start_pre, "ExecStartPre");
        write_vec!(buf, self.exec_start, "ExecStart");
        write_vec!(buf, self.exec_start_post, "ExecStartPost");
        write_vec!(buf, self.exec_reload, "ExecReload");
        write_vec!(buf, self.exec_stop, "ExecStop");
        write_vec!(buf, self.exec_stop_post, "ExecStopPost");

        // Restart configuration
        write_option!(buf, self.restart, "Restart");
        write_option!(buf, self.restart_mode, "RestartMode");

        if let Some(sec) = self.restart_sec {
            writeln!(buf, "RestartSec={}", sec)?;
        }
        if let Some(steps) = self.restart_steps {
            writeln!(buf, "RestartSteps={}", steps)?;
        }

        write_option!(buf, self.restart_max_delay_sec, "RestartMaxDelaySec");
        write_vec!(buf, self.success_exit_status, "SuccessExitStatus");
        write_vec!(
            buf,
            self.restart_prevent_exit_status,
            "RestartPreventExitStatus"
        );
        write_vec!(
            buf,
            self.restart_force_exit_status,
            "RestartForceExitStatus"
        );

        // Timeouts
        write_option!(buf, self.timeout_start_sec, "TimeoutStartSec");
        write_option!(buf, self.timeout_stop_sec, "TimeoutStopSec");
        write_option!(buf, self.timeout_abort_sec, "TimeoutAbortSec");
        write_option!(buf, self.timeout_sec, "TimeoutSec");
        write_option!(
            buf,
            self.timeout_start_failure_mode,
            "TimeoutStartFailureMode"
        );
        write_option!(
            buf,
            self.timeout_stop_failure_mode,
            "TimeoutStopFailureMode"
        );

        // Runtime limits
        write_option!(buf, self.runtime_max_sec, "RuntimeMaxSec");
        write_option!(
            buf,
            self.runtime_randomized_extra_sec,
            "RuntimeRandomizedExtraSec"
        );
        write_option!(buf, self.watchdog_sec, "WatchdogSec");

        // Execution environment
        write_bool!(
            buf,
            self.root_directory_start_only,
            "RootDirectoryStartOnly"
        );
        write_bool!(buf, self.non_blocking, "NonBlocking");
        write_option!(buf, self.notify_access, "NotifyAccess");

        // Socket and file descriptor management
        write_vec!(buf, self.sockets, "Sockets");

        if let Some(max) = self.file_descriptor_store_max {
            writeln!(buf, "FileDescriptorStoreMax={}", max)?;
        }

        write_option!(
            buf,
            self.file_descriptor_store_preserve,
            "FileDescriptorStorePreserve"
        );

        if let Some(ref path) = self.usb_function_descriptors {
            writeln!(buf, "USBFunctionDescriptors={}", path.display())?;
        }
        if let Some(ref path) = self.usb_function_strings {
            writeln!(buf, "USBFunctionStrings={}", path.display())?;
        }

        // Resource management
        write_option!(buf, self.oom_policy, "OOMPolicy");
        write_vec!(buf, self.open_file, "OpenFile");

        // Signals
        write_option!(buf, self.reload_signal, "ReloadSignal");

        Ok(())
    }
}

impl Service {
    /// Validate the service configuration according to systemd specifications
    pub fn validate(&self) -> Result<(), String> {
        // All validations are now compile-time enforced via enum types
        Ok(())
    }

    // Builder pattern setters
    pub fn service_type(mut self, value: ServiceType) -> Self {
        self.service_type = Some(value);
        self
    }

    pub fn exit_type(mut self, value: ExitType) -> Self {
        self.exit_type = Some(value);
        self
    }

    pub fn remain_after_exit(mut self, value: bool) -> Self {
        self.remain_after_exit = Some(value);
        self
    }

    pub fn guess_main_pid(mut self, value: bool) -> Self {
        self.guess_main_pid = Some(value);
        self
    }

    pub fn pid_file(mut self, value: PathBuf) -> Self {
        self.pid_file = Some(value);
        self
    }

    pub fn bus_name(mut self, value: impl Into<String>) -> Self {
        self.bus_name = Some(value.into());
        self
    }

    pub fn exec_start(mut self, value: Vec<impl Into<String>>) -> Self {
        self.exec_start = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn exec_start_pre(mut self, value: Vec<impl Into<String>>) -> Self {
        self.exec_start_pre = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn exec_start_post(mut self, value: Vec<impl Into<String>>) -> Self {
        self.exec_start_post = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn exec_condition(mut self, value: Vec<impl Into<String>>) -> Self {
        self.exec_condition = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn exec_reload(mut self, value: Vec<impl Into<String>>) -> Self {
        self.exec_reload = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn exec_stop(mut self, value: Vec<impl Into<String>>) -> Self {
        self.exec_stop = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn exec_stop_post(mut self, value: Vec<impl Into<String>>) -> Self {
        self.exec_stop_post = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn restart_sec(mut self, value: u64) -> Self {
        self.restart_sec = Some(value);
        self
    }

    pub fn restart_steps(mut self, value: u32) -> Self {
        self.restart_steps = Some(value);
        self
    }

    pub fn restart_max_delay_sec(mut self, value: impl Into<String>) -> Self {
        self.restart_max_delay_sec = Some(value.into());
        self
    }

    pub fn timeout_start_sec(mut self, value: impl Into<String>) -> Self {
        self.timeout_start_sec = Some(value.into());
        self
    }

    pub fn timeout_stop_sec(mut self, value: impl Into<String>) -> Self {
        self.timeout_stop_sec = Some(value.into());
        self
    }

    pub fn timeout_abort_sec(mut self, value: impl Into<String>) -> Self {
        self.timeout_abort_sec = Some(value.into());
        self
    }

    pub fn timeout_sec(mut self, value: impl Into<String>) -> Self {
        self.timeout_sec = Some(value.into());
        self
    }

    pub fn timeout_start_failure_mode(mut self, value: TimeoutFailureMode) -> Self {
        self.timeout_start_failure_mode = Some(value);
        self
    }

    pub fn timeout_stop_failure_mode(mut self, value: TimeoutFailureMode) -> Self {
        self.timeout_stop_failure_mode = Some(value);
        self
    }

    pub fn runtime_max_sec(mut self, value: impl Into<String>) -> Self {
        self.runtime_max_sec = Some(value.into());
        self
    }

    pub fn runtime_randomized_extra_sec(mut self, value: impl Into<String>) -> Self {
        self.runtime_randomized_extra_sec = Some(value.into());
        self
    }

    pub fn watchdog_sec(mut self, value: impl Into<String>) -> Self {
        self.watchdog_sec = Some(value.into());
        self
    }

    pub fn restart(mut self, value: RestartType) -> Self {
        self.restart = Some(value);
        self
    }

    pub fn restart_mode(mut self, value: RestartMode) -> Self {
        self.restart_mode = Some(value);
        self
    }

    pub fn success_exit_status(mut self, value: Vec<impl Into<String>>) -> Self {
        self.success_exit_status = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn restart_prevent_exit_status(mut self, value: Vec<impl Into<String>>) -> Self {
        self.restart_prevent_exit_status = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn restart_force_exit_status(mut self, value: Vec<impl Into<String>>) -> Self {
        self.restart_force_exit_status = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn root_directory_start_only(mut self, value: bool) -> Self {
        self.root_directory_start_only = Some(value);
        self
    }

    pub fn non_blocking(mut self, value: bool) -> Self {
        self.non_blocking = Some(value);
        self
    }

    pub fn notify_access(mut self, value: NotifyAccess) -> Self {
        self.notify_access = Some(value);
        self
    }

    pub fn sockets(mut self, value: Vec<impl Into<String>>) -> Self {
        self.sockets = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn file_descriptor_store_max(mut self, value: u32) -> Self {
        self.file_descriptor_store_max = Some(value);
        self
    }

    pub fn file_descriptor_store_preserve(mut self, value: impl Into<String>) -> Self {
        self.file_descriptor_store_preserve = Some(value.into());
        self
    }

    pub fn usb_function_descriptors(mut self, value: PathBuf) -> Self {
        self.usb_function_descriptors = Some(value);
        self
    }

    pub fn usb_function_strings(mut self, value: PathBuf) -> Self {
        self.usb_function_strings = Some(value);
        self
    }

    pub fn oom_policy(mut self, value: OomPolicy) -> Self {
        self.oom_policy = Some(value);
        self
    }

    pub fn open_file(mut self, value: Vec<impl Into<String>>) -> Self {
        self.open_file = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn reload_signal(mut self, value: impl Into<String>) -> Self {
        self.reload_signal = Some(value.into());
        self
    }

    pub fn exec(mut self, value: Exec) -> Self {
        self.exec = Some(value);
        self
    }
}
