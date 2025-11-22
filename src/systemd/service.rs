use std::fmt;
use std::path::PathBuf;

/// Service section options
/// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#Options
#[derive(Debug, Clone, Default)]
pub struct Service {
    /// Configures the mechanism via which the service notifies the manager that the service start-up has finished.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#Type=
    /// One of: simple, exec, forking, oneshot, dbus, notify, notify-reload, or idle
    pub service_type: Option<String>,

    /// Specifies when the manager should consider the service to be finished.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ExitType=
    /// One of: main or cgroup
    pub exit_type: Option<String>,

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
    pub timeout_start_failure_mode: Option<String>,

    /// Action taken when stop timeout is hit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#TimeoutStopFailureMode=
    pub timeout_stop_failure_mode: Option<String>,

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
    pub restart: Option<String>,

    /// How a service should restart.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#RestartMode=
    /// One of: normal, direct, or debug
    pub restart_mode: Option<String>,

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
    pub notify_access: Option<String>,

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
    pub oom_policy: Option<String>,

    /// Files or sockets to open and pass to the service.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#OpenFile=
    pub open_file: Option<Vec<String>>,

    /// UNIX process signal to send when reloading the service.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#ReloadSignal=
    pub reload_signal: Option<String>,
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
        write_vec!(buf, self.restart_prevent_exit_status, "RestartPreventExitStatus");
        write_vec!(buf, self.restart_force_exit_status, "RestartForceExitStatus");

        // Timeouts
        write_option!(buf, self.timeout_start_sec, "TimeoutStartSec");
        write_option!(buf, self.timeout_stop_sec, "TimeoutStopSec");
        write_option!(buf, self.timeout_abort_sec, "TimeoutAbortSec");
        write_option!(buf, self.timeout_sec, "TimeoutSec");
        write_option!(buf, self.timeout_start_failure_mode, "TimeoutStartFailureMode");
        write_option!(buf, self.timeout_stop_failure_mode, "TimeoutStopFailureMode");

        // Runtime limits
        write_option!(buf, self.runtime_max_sec, "RuntimeMaxSec");
        write_option!(buf, self.runtime_randomized_extra_sec, "RuntimeRandomizedExtraSec");
        write_option!(buf, self.watchdog_sec, "WatchdogSec");

        // Execution environment
        write_bool!(buf, self.root_directory_start_only, "RootDirectoryStartOnly");
        write_bool!(buf, self.non_blocking, "NonBlocking");
        write_option!(buf, self.notify_access, "NotifyAccess");

        // Socket and file descriptor management
        write_vec!(buf, self.sockets, "Sockets");
        
        if let Some(max) = self.file_descriptor_store_max {
            writeln!(buf, "FileDescriptorStoreMax={}", max)?;
        }
        
        write_option!(buf, self.file_descriptor_store_preserve, "FileDescriptorStorePreserve");
        
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