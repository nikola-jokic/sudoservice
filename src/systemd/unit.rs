use std::fmt;
use std::path::PathBuf;

/// Unit section options
/// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#%5BUnit%5D%20Section%20Options
#[derive(Debug, Clone, Default)]
pub struct Unit {
    /// A brief, meaningful, human-readable text identifying the unit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#Description=
    pub description: Option<String>,

    /// A space-separated list of URIs referencing documentation for this unit or its configuration.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#Documentation=
    /// Accepted are only URIs of the types "http://", "https://", "file:", "info:", "man:".
    pub documentation: Option<Vec<String>>,

    /// Configures (weak) requirement dependencies on other units.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#Wants=
    pub wants: Option<Vec<String>>,

    /// Configures requirement dependencies on other units.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#Requires=
    pub requires: Option<Vec<String>>,

    /// Similar to Requires=, but must be already active when this unit is started.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#Requisite=
    pub requisite: Option<Vec<String>>,

    /// Configures requirement dependencies similar to Requires= but stops this unit if the bound unit stops.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#BindsTo=
    pub binds_to: Option<Vec<String>>,

    /// Configures dependencies similar to Requires=, limited to stopping and restarting.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#PartOf=
    pub part_of: Option<Vec<String>>,

    /// Continuously restarts the listed units if they become inactive or failed.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#Upholds=
    pub upholds: Option<Vec<String>>,

    /// Configures negative requirement dependencies.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#Conflicts=
    pub conflicts: Option<Vec<String>>,

    /// Units listed will be started before this unit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#Before=
    pub before: Option<Vec<String>>,

    /// Units listed will be started after this unit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#After=
    pub after: Option<Vec<String>>,

    /// Units to activate when this unit enters the "failed" state.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#OnFailure=
    pub on_failure: Option<Vec<String>>,

    /// Units to activate when this unit enters the "inactive" state.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#OnSuccess=
    pub on_success: Option<Vec<String>>,

    /// Units to which reload requests from this unit shall be propagated to.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#PropagatesReloadTo=
    pub propagates_reload_to: Option<Vec<String>>,

    /// Units from which reload requests shall be propagated to this unit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ReloadPropagatedFrom=
    pub reload_propagated_from: Option<Vec<String>>,

    /// Units to which stop requests from this unit shall be propagated to.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#PropagatesStopTo=
    pub propagates_stop_to: Option<Vec<String>>,

    /// Units from which stop requests shall be propagated to this unit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#StopPropagatedFrom=
    pub stop_propagated_from: Option<Vec<String>>,

    /// Network and/or temporary file namespace to join from listed units.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#JoinsNamespaceOf=
    pub joins_namespace_of: Option<Vec<String>>,

    /// Paths that must be accessible for this unit to work.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#RequiresMountsFor=
    pub requires_mounts_for: Option<Vec<String>>,

    /// Same as RequiresMountsFor= but adds Wants= dependencies instead of Requires=.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#WantsMountsFor=
    pub wants_mounts_for: Option<Vec<String>>,

    /// Job mode for OnSuccess=/OnFailure= units.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#OnSuccessJobMode=
    pub on_success_job_mode: Option<String>,

    /// Job mode for OnFailure= units.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#OnFailureJobMode=
    pub on_failure_job_mode: Option<String>,

    /// If true, this unit will not be stopped when isolating another unit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#IgnoreOnIsolate=
    pub ignore_on_isolate: Option<bool>,

    /// If true, this unit will be stopped when it is no longer used.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#StopWhenUnneeded=
    pub stop_when_unneeded: Option<bool>,

    /// If true, explicit start-up requested by the user is denied.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#RefuseManualStart=
    pub refuse_manual_start: Option<bool>,

    /// If true, explicit termination requested by the user is denied.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#RefuseManualStop=
    pub refuse_manual_stop: Option<bool>,

    /// If true, this unit may be used with the systemctl isolate command.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AllowIsolate=
    pub allow_isolate: Option<bool>,

    /// If false, default dependencies will not be created.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#DefaultDependencies=
    pub default_dependencies: Option<bool>,

    /// If true, processes will not be sent SIGTERM/SIGKILL during final system shutdown.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#SurviveFinalKillSignal=
    pub survive_final_kill_signal: Option<bool>,

    /// Tweaks the garbage collection algorithm for this unit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#CollectMode=
    pub collect_mode: Option<String>,

    /// Action to take when the unit stops and enters a failed state.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#FailureAction=
    pub failure_action: Option<String>,

    /// Action to take when the unit enters an inactive state.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#SuccessAction=
    pub success_action: Option<String>,

    /// Exit status to propagate when FailureAction= is set to exit or exit-force.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#FailureActionExitStatus=
    pub failure_action_exit_status: Option<u8>,

    /// Exit status to propagate when SuccessAction= is set to exit or exit-force.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#SuccessActionExitStatus=
    pub success_action_exit_status: Option<u8>,

    /// Timeout for the whole job.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#JobTimeoutSec=
    pub job_timeout_sec: Option<u64>,

    /// Timeout that starts when the job is actually started.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#JobRunningTimeoutSec=
    pub job_running_timeout_sec: Option<u64>,

    /// Action to take when the job timeout is hit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#JobTimeoutAction=
    pub job_timeout_action: Option<String>,

    /// Reboot string to pass to reboot(2) when job timeout is hit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#JobTimeoutRebootArgument=
    pub job_timeout_reboot_argument: Option<String>,

    /// Time interval for start rate limiting.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#StartLimitIntervalSec=
    pub start_limit_interval_sec: Option<u64>,

    /// Number of allowed starts per interval.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#StartLimitBurst=
    pub start_limit_burst: Option<u32>,

    /// Action to take if the start rate limit is hit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#StartLimitAction=
    pub start_limit_action: Option<String>,

    /// Argument for the reboot(2) system call if StartLimitAction= is a reboot action.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#RebootArgument=
    pub reboot_argument: Option<String>,

    /// Path to a configuration file this unit has been generated from.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#SourcePath=
    pub source_path: Option<PathBuf>,

    // Condition checks
    /// Check whether the system is running on a specific architecture.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionArchitecture=
    pub condition_architecture: Option<String>,

    /// Check whether the system's firmware is of a certain type.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionFirmware=
    pub condition_firmware: Option<String>,

    /// Check whether the system is executed in a virtualized environment.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionVirtualization=
    pub condition_virtualization: Option<String>,

    /// Match against hostname, machine ID, boot ID or product UUID.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionHost=
    pub condition_host: Option<String>,

    /// Check whether a specific kernel command line option is set.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionKernelCommandLine=
    pub condition_kernel_command_line: Option<String>,

    /// Check whether the kernel version matches a certain expression.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionKernelVersion=
    pub condition_kernel_version: Option<String>,

    /// Check whether a credential by the specified name was passed in.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionCredential=
    pub condition_credential: Option<String>,

    /// Check whether a specific environment variable is set.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionEnvironment=
    pub condition_environment: Option<String>,

    /// Check whether the given security technology is enabled.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionSecurity=
    pub condition_security: Option<Vec<String>>,

    /// Check whether the given capability exists in the capability bounding set.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionCapability=
    pub condition_capability: Option<Vec<String>>,

    /// Check whether the system has AC power.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionACPower=
    pub condition_ac_power: Option<bool>,

    /// Check whether /var/ or /etc/ requires an update.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionNeedsUpdate=
    pub condition_needs_update: Option<String>,

    /// Check whether the system is booting up for the first time.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionFirstBoot=
    pub condition_first_boot: Option<bool>,

    /// Check for the existence of a file.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionPathExists=
    pub condition_path_exists: Option<String>,

    /// Check for the existence of at least one file matching a glob pattern.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionPathExistsGlob=
    pub condition_path_exists_glob: Option<String>,

    /// Check that a path exists and is a directory.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionPathIsDirectory=
    pub condition_path_is_directory: Option<String>,

    /// Check that a path exists and is a symbolic link.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionPathIsSymbolicLink=
    pub condition_path_is_symbolic_link: Option<String>,

    /// Check that a path exists and is a mount point.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionPathIsMountPoint=
    pub condition_path_is_mount_point: Option<String>,

    /// Check that the underlying file system is readable and writable.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionPathIsReadWrite=
    pub condition_path_is_read_write: Option<String>,

    /// Check that the underlying file system's backing block device is encrypted.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionPathIsEncrypted=
    pub condition_path_is_encrypted: Option<String>,

    /// Check that a path exists and is a non-empty directory.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionDirectoryNotEmpty=
    pub condition_directory_not_empty: Option<String>,

    /// Check that a path exists and refers to a regular file with non-zero size.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionFileNotEmpty=
    pub condition_file_not_empty: Option<String>,

    /// Check that a path exists, is a regular file, and marked executable.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionFileIsExecutable=
    pub condition_file_is_executable: Option<String>,

    /// Check whether the service manager is running as the given user.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionUser=
    pub condition_user: Option<String>,

    /// Check whether the service manager's group matches the specified group or GID.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionGroup=
    pub condition_group: Option<String>,

    /// Check whether given cgroup controllers are available for use.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionControlGroupController=
    pub condition_control_group_controller: Option<Vec<String>>,

    /// Verify that the specified amount of system memory is available.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionMemory=
    pub condition_memory: Option<String>,

    /// Verify that the specified number of CPUs is available.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionCPUs=
    pub condition_cpus: Option<String>,

    /// Verify that a given CPU feature is available.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionCPUFeature=
    pub condition_cpu_feature: Option<Vec<String>>,

    /// Verify that a specific "key=value" pair is set in os-release.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionOSRelease=
    pub condition_os_release: Option<String>,

    /// Verify that memory pressure is below or equal to a threshold.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionMemoryPressure=
    pub condition_memory_pressure: Option<String>,

    /// Verify that CPU pressure is below or equal to a threshold.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionCPUPressure=
    pub condition_cpu_pressure: Option<String>,

    /// Verify that IO pressure is below or equal to a threshold.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionIOPressure=
    pub condition_io_pressure: Option<String>,

    // Assert checks (similar to conditions but cause failure instead of skip)
    /// Assert that the system is running on a specific architecture.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertArchitecture=
    pub assert_architecture: Option<String>,

    /// Assert that the system's firmware is of a certain type.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertFirmware=
    pub assert_firmware: Option<String>,

    /// Assert that the system is executed in a virtualized environment.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertVirtualization=
    pub assert_virtualization: Option<String>,

    /// Assert hostname, machine ID, boot ID or product UUID.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertHost=
    pub assert_host: Option<String>,

    /// Assert that a specific kernel command line option is set.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertKernelCommandLine=
    pub assert_kernel_command_line: Option<String>,

    /// Assert that the kernel version matches a certain expression.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertKernelVersion=
    pub assert_kernel_version: Option<String>,

    /// Assert that a credential by the specified name was passed in.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertCredential=
    pub assert_credential: Option<String>,

    /// Assert that a specific environment variable is set.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertEnvironment=
    pub assert_environment: Option<String>,

    /// Assert that the given security technology is enabled.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertSecurity=
    pub assert_security: Option<Vec<String>>,

    /// Assert that the given capability exists in the capability bounding set.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertCapability=
    pub assert_capability: Option<Vec<String>>,

    /// Assert that the system has AC power.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertACPower=
    pub assert_ac_power: Option<bool>,

    /// Assert that /var/ or /etc/ requires an update.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertNeedsUpdate=
    pub assert_needs_update: Option<String>,

    /// Assert that the system is booting up for the first time.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertFirstBoot=
    pub assert_first_boot: Option<bool>,

    /// Assert the existence of a file.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertPathExists=
    pub assert_path_exists: Option<String>,

    /// Assert the existence of at least one file matching a glob pattern.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertPathExistsGlob=
    pub assert_path_exists_glob: Option<String>,

    /// Assert that a path exists and is a directory.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertPathIsDirectory=
    pub assert_path_is_directory: Option<String>,

    /// Assert that a path exists and is a symbolic link.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertPathIsSymbolicLink=
    pub assert_path_is_symbolic_link: Option<String>,

    /// Assert that a path exists and is a mount point.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertPathIsMountPoint=
    pub assert_path_is_mount_point: Option<String>,

    /// Assert that the underlying file system is readable and writable.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertPathIsReadWrite=
    pub assert_path_is_read_write: Option<String>,

    /// Assert that the underlying file system's backing block device is encrypted.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertPathIsEncrypted=
    pub assert_path_is_encrypted: Option<String>,

    /// Assert that a path exists and is a non-empty directory.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertDirectoryNotEmpty=
    pub assert_directory_not_empty: Option<String>,

    /// Assert that a path exists and refers to a regular file with non-zero size.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertFileNotEmpty=
    pub assert_file_not_empty: Option<String>,

    /// Assert that a path exists, is a regular file, and marked executable.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertFileIsExecutable=
    pub assert_file_is_executable: Option<String>,

    /// Assert that the service manager is running as the given user.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertUser=
    pub assert_user: Option<String>,

    /// Assert that the service manager's group matches the specified group or GID.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertGroup=
    pub assert_group: Option<String>,

    /// Assert that given cgroup controllers are available for use.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertControlGroupController=
    pub assert_control_group_controller: Option<Vec<String>>,

    /// Assert that the specified amount of system memory is available.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertMemory=
    pub assert_memory: Option<String>,

    /// Assert that the specified number of CPUs is available.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertCPUs=
    pub assert_cpus: Option<String>,

    /// Assert that a given CPU feature is available.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertCPUFeature=
    pub assert_cpu_feature: Option<Vec<String>>,

    /// Assert that a specific "key=value" pair is set in os-release.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertOSRelease=
    pub assert_os_release: Option<String>,

    /// Assert that memory pressure is below or equal to a threshold.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertMemoryPressure=
    pub assert_memory_pressure: Option<String>,

    /// Assert that CPU pressure is below or equal to a threshold.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertCPUPressure=
    pub assert_cpu_pressure: Option<String>,

    /// Assert that IO pressure is below or equal to a threshold.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertIOPressure=
    pub assert_io_pressure: Option<String>,
}

impl fmt::Display for Unit {
    fn fmt(&self, buf: &mut fmt::Formatter<'_>) -> fmt::Result {
        buf.write_str("[Unit]\n")?;
        // Basic metadata
        write_option!(buf, self.description, "Description");
        write_vec!(buf, self.documentation, "Documentation");

        // Dependencies
        write_vec!(buf, self.wants, "Wants");
        write_vec!(buf, self.requires, "Requires");
        write_vec!(buf, self.requisite, "Requisite");
        write_vec!(buf, self.binds_to, "BindsTo");
        write_vec!(buf, self.part_of, "PartOf");
        write_vec!(buf, self.upholds, "Upholds");
        write_vec!(buf, self.conflicts, "Conflicts");

        // Ordering
        write_vec!(buf, self.before, "Before");
        write_vec!(buf, self.after, "After");

        // Lifecycle actions
        write_vec!(buf, self.on_failure, "OnFailure");
        write_vec!(buf, self.on_success, "OnSuccess");
        write_vec!(buf, self.propagates_reload_to, "PropagatesReloadTo");
        write_vec!(buf, self.reload_propagated_from, "ReloadPropagatedFrom");
        write_vec!(buf, self.propagates_stop_to, "PropagatesStopTo");
        write_vec!(buf, self.stop_propagated_from, "StopPropagatedFrom");

        // Namespace & mounts
        write_vec!(buf, self.joins_namespace_of, "JoinsNamespaceOf");
        write_vec!(buf, self.requires_mounts_for, "RequiresMountsFor");
        write_vec!(buf, self.wants_mounts_for, "WantsMountsFor");

        // Job modes
        write_option!(buf, self.on_success_job_mode, "OnSuccessJobMode");
        write_option!(buf, self.on_failure_job_mode, "OnFailureJobMode");

        // Behavioral flags
        write_bool!(buf, self.ignore_on_isolate, "IgnoreOnIsolate");
        write_bool!(buf, self.stop_when_unneeded, "StopWhenUnneeded");
        write_bool!(buf, self.refuse_manual_start, "RefuseManualStart");
        write_bool!(buf, self.refuse_manual_stop, "RefuseManualStop");
        write_bool!(buf, self.allow_isolate, "AllowIsolate");
        write_bool!(buf, self.default_dependencies, "DefaultDependencies");
        write_bool!(
            buf,
            self.survive_final_kill_signal,
            "SurviveFinalKillSignal"
        );

        // Actions & modes
        write_option!(buf, self.collect_mode, "CollectMode");
        write_option!(buf, self.failure_action, "FailureAction");
        write_option!(buf, self.success_action, "SuccessAction");

        if let Some(status) = self.failure_action_exit_status {
            writeln!(buf, "FailureActionExitStatus={}", status)?;
        }
        if let Some(status) = self.success_action_exit_status {
            writeln!(buf, "SuccessActionExitStatus={}", status)?;
        }

        // Timeouts
        if let Some(timeout) = self.job_timeout_sec {
            writeln!(buf, "JobTimeoutSec={}", timeout)?;
        }
        if let Some(timeout) = self.job_running_timeout_sec {
            writeln!(buf, "JobRunningTimeoutSec={}", timeout)?;
        }
        write_option!(buf, self.job_timeout_action, "JobTimeoutAction");
        write_option!(
            buf,
            self.job_timeout_reboot_argument,
            "JobTimeoutRebootArgument"
        );

        // Start rate limiting
        if let Some(interval) = self.start_limit_interval_sec {
            writeln!(buf, "StartLimitIntervalSec={}", interval)?;
        }
        if let Some(burst) = self.start_limit_burst {
            writeln!(buf, "StartLimitBurst={}", burst)?;
        }
        write_option!(buf, self.start_limit_action, "StartLimitAction");
        write_option!(buf, self.reboot_argument, "RebootArgument");

        // Source path
        if let Some(ref path) = self.source_path {
            writeln!(buf, "SourcePath={}", path.display())?;
        }

        // Condition checks
        write_option!(buf, self.condition_architecture, "ConditionArchitecture");
        write_option!(buf, self.condition_firmware, "ConditionFirmware");
        write_option!(
            buf,
            self.condition_virtualization,
            "ConditionVirtualization"
        );
        write_option!(buf, self.condition_host, "ConditionHost");
        write_option!(
            buf,
            self.condition_kernel_command_line,
            "ConditionKernelCommandLine"
        );
        write_option!(buf, self.condition_kernel_version, "ConditionKernelVersion");
        write_option!(buf, self.condition_credential, "ConditionCredential");
        write_option!(buf, self.condition_environment, "ConditionEnvironment");
        write_vec!(buf, self.condition_security, "ConditionSecurity");
        write_vec!(buf, self.condition_capability, "ConditionCapability");
        write_bool!(buf, self.condition_ac_power, "ConditionACPower");
        write_option!(buf, self.condition_needs_update, "ConditionNeedsUpdate");
        write_bool!(buf, self.condition_first_boot, "ConditionFirstBoot");
        write_option!(buf, self.condition_path_exists, "ConditionPathExists");
        write_option!(
            buf,
            self.condition_path_exists_glob,
            "ConditionPathExistsGlob"
        );
        write_option!(
            buf,
            self.condition_path_is_directory,
            "ConditionPathIsDirectory"
        );
        write_option!(
            buf,
            self.condition_path_is_symbolic_link,
            "ConditionPathIsSymbolicLink"
        );
        write_option!(
            buf,
            self.condition_path_is_mount_point,
            "ConditionPathIsMountPoint"
        );
        write_option!(
            buf,
            self.condition_path_is_read_write,
            "ConditionPathIsReadWrite"
        );
        write_option!(
            buf,
            self.condition_path_is_encrypted,
            "ConditionPathIsEncrypted"
        );
        write_option!(
            buf,
            self.condition_directory_not_empty,
            "ConditionDirectoryNotEmpty"
        );
        write_option!(buf, self.condition_file_not_empty, "ConditionFileNotEmpty");
        write_option!(
            buf,
            self.condition_file_is_executable,
            "ConditionFileIsExecutable"
        );
        write_option!(buf, self.condition_user, "ConditionUser");
        write_option!(buf, self.condition_group, "ConditionGroup");
        write_vec!(
            buf,
            self.condition_control_group_controller,
            "ConditionControlGroupController"
        );
        write_option!(buf, self.condition_memory, "ConditionMemory");
        write_option!(buf, self.condition_cpus, "ConditionCPUs");
        write_vec!(buf, self.condition_cpu_feature, "ConditionCPUFeature");
        write_option!(buf, self.condition_os_release, "ConditionOSRelease");
        write_option!(
            buf,
            self.condition_memory_pressure,
            "ConditionMemoryPressure"
        );
        write_option!(buf, self.condition_cpu_pressure, "ConditionCPUPressure");
        write_option!(buf, self.condition_io_pressure, "ConditionIOPressure");

        // Assert checks
        write_option!(buf, self.assert_architecture, "AssertArchitecture");
        write_option!(buf, self.assert_firmware, "AssertFirmware");
        write_option!(buf, self.assert_virtualization, "AssertVirtualization");
        write_option!(buf, self.assert_host, "AssertHost");
        write_option!(
            buf,
            self.assert_kernel_command_line,
            "AssertKernelCommandLine"
        );
        write_option!(buf, self.assert_kernel_version, "AssertKernelVersion");
        write_option!(buf, self.assert_credential, "AssertCredential");
        write_option!(buf, self.assert_environment, "AssertEnvironment");
        write_vec!(buf, self.assert_security, "AssertSecurity");
        write_vec!(buf, self.assert_capability, "AssertCapability");
        write_bool!(buf, self.assert_ac_power, "AssertACPower");
        write_option!(buf, self.assert_needs_update, "AssertNeedsUpdate");
        write_bool!(buf, self.assert_first_boot, "AssertFirstBoot");
        write_option!(buf, self.assert_path_exists, "AssertPathExists");
        write_option!(buf, self.assert_path_exists_glob, "AssertPathExistsGlob");
        write_option!(buf, self.assert_path_is_directory, "AssertPathIsDirectory");
        write_option!(
            buf,
            self.assert_path_is_symbolic_link,
            "AssertPathIsSymbolicLink"
        );
        write_option!(
            buf,
            self.assert_path_is_mount_point,
            "AssertPathIsMountPoint"
        );
        write_option!(buf, self.assert_path_is_read_write, "AssertPathIsReadWrite");
        write_option!(buf, self.assert_path_is_encrypted, "AssertPathIsEncrypted");
        write_option!(
            buf,
            self.assert_directory_not_empty,
            "AssertDirectoryNotEmpty"
        );
        write_option!(buf, self.assert_file_not_empty, "AssertFileNotEmpty");
        write_option!(
            buf,
            self.assert_file_is_executable,
            "AssertFileIsExecutable"
        );
        write_option!(buf, self.assert_user, "AssertUser");
        write_option!(buf, self.assert_group, "AssertGroup");
        write_vec!(
            buf,
            self.assert_control_group_controller,
            "AssertControlGroupController"
        );
        write_option!(buf, self.assert_memory, "AssertMemory");
        write_option!(buf, self.assert_cpus, "AssertCPUs");
        write_vec!(buf, self.assert_cpu_feature, "AssertCPUFeature");
        write_option!(buf, self.assert_os_release, "AssertOSRelease");
        write_option!(buf, self.assert_memory_pressure, "AssertMemoryPressure");
        write_option!(buf, self.assert_cpu_pressure, "AssertCPUPressure");
        write_option!(buf, self.assert_io_pressure, "AssertIOPressure");

        Ok(())
    }
}

impl Unit {
    /// Validate the unit configuration according to systemd specifications
    pub fn validate(&self) -> Result<(), String> {
        // Documentation URIs must be of allowed types
        if let Some(ref docs) = self.documentation {
            for doc in docs {
                if !doc.starts_with("http://") 
                    && !doc.starts_with("https://") 
                    && !doc.starts_with("file:") 
                    && !doc.starts_with("info:") 
                    && !doc.starts_with("man:") {
                    return Err(format!("Invalid documentation URI '{}': must start with http://, https://, file:, info:, or man:", doc));
                }
            }
        }

        // Job modes
        const JOB_MODES: &[&str] = &["fail", "replace", "replace-irreversibly", "isolate", "flush", "ignore-dependencies", "ignore-requirements"];
        if let Some(ref mode) = self.on_success_job_mode {
            if !JOB_MODES.contains(&mode.as_str()) {
                return Err(format!("Invalid OnSuccessJobMode '{}': must be one of {:?}", mode, JOB_MODES));
            }
        }
        if let Some(ref mode) = self.on_failure_job_mode {
            if !JOB_MODES.contains(&mode.as_str()) {
                return Err(format!("Invalid OnFailureJobMode '{}': must be one of {:?}", mode, JOB_MODES));
            }
        }

        // Collect mode
        const COLLECT_MODES: &[&str] = &["inactive", "inactive-or-failed"];
        if let Some(ref mode) = self.collect_mode {
            if !COLLECT_MODES.contains(&mode.as_str()) {
                return Err(format!("Invalid CollectMode '{}': must be one of {:?}", mode, COLLECT_MODES));
            }
        }

        // Failure/Success actions
        const ACTIONS: &[&str] = &["none", "reboot", "reboot-force", "reboot-immediate", "poweroff", "poweroff-force", "poweroff-immediate", "exit", "exit-force", "soft-reboot", "soft-reboot-force", "kexec", "kexec-force", "halt", "halt-force", "halt-immediate"];
        if let Some(ref action) = self.failure_action {
            if !ACTIONS.contains(&action.as_str()) {
                return Err(format!("Invalid FailureAction '{}': must be one of {:?}", action, ACTIONS));
            }
        }
        if let Some(ref action) = self.success_action {
            if !ACTIONS.contains(&action.as_str()) {
                return Err(format!("Invalid SuccessAction '{}': must be one of {:?}", action, ACTIONS));
            }
        }

        // Job timeout action
        if let Some(ref action) = self.job_timeout_action {
            if !ACTIONS.contains(&action.as_str()) {
                return Err(format!("Invalid JobTimeoutAction '{}': must be one of {:?}", action, ACTIONS));
            }
        }

        // Start limit action
        if let Some(ref action) = self.start_limit_action {
            if !ACTIONS.contains(&action.as_str()) {
                return Err(format!("Invalid StartLimitAction '{}': must be one of {:?}", action, ACTIONS));
            }
        }

        // Condition architectures
        const ARCHITECTURES: &[&str] = &["x86", "x86-64", "ppc", "ppc-le", "ppc64", "ppc64-le", "ia64", "parisc", "parisc64", "s390", "s390x", "sparc", "sparc64", "mips", "mips-le", "mips64", "mips64-le", "alpha", "arm", "arm-be", "arm64", "arm64-be", "sh", "sh64", "m68k", "tilegx", "cris", "arc", "arc-be", "native"];
        if let Some(ref arch) = self.condition_architecture {
            if !ARCHITECTURES.contains(&arch.as_str()) {
                return Err(format!("Invalid ConditionArchitecture '{}': must be one of {:?}", arch, ARCHITECTURES));
            }
        }
        if let Some(ref arch) = self.assert_architecture {
            if !ARCHITECTURES.contains(&arch.as_str()) {
                return Err(format!("Invalid AssertArchitecture '{}': must be one of {:?}", arch, ARCHITECTURES));
            }
        }

        // Condition firmware
        // This is more complex, but for now skip detailed validation

        // Condition virtualization
        const VIRTUALIZATIONS: &[&str] = &["vm", "container", "qemu", "kvm", "amazon", "zvm", "vmware", "microsoft", "oracle", "powervm", "xen", "bochs", "uml", "bhyve", "qnx", "apple", "sre", "openvz", "lxc", "lxc-libvirt", "systemd-nspawn", "docker", "podman", "rkt", "wsl", "proot", "pouch", "acrn", "private-users"];
        if let Some(ref virt) = self.condition_virtualization {
            if !VIRTUALIZATIONS.contains(&virt.as_str()) {
                return Err(format!("Invalid ConditionVirtualization '{}': must be one of {:?}", virt, VIRTUALIZATIONS));
            }
        }
        if let Some(ref virt) = self.assert_virtualization {
            if !VIRTUALIZATIONS.contains(&virt.as_str()) {
                return Err(format!("Invalid AssertVirtualization '{}': must be one of {:?}", virt, VIRTUALIZATIONS));
            }
        }

        // Condition security
        const SECURITY_TECHS: &[&str] = &["selinux", "apparmor", "tomoyo", "smack", "ima", "audit", "uefi-secureboot", "tpm2", "cvm", "measured-uki"];
        if let Some(ref secs) = self.condition_security {
            for sec in secs {
                if !SECURITY_TECHS.contains(&sec.as_str()) {
                    return Err(format!("Invalid ConditionSecurity '{}': must be one of {:?}", sec, SECURITY_TECHS));
                }
            }
        }
        if let Some(ref secs) = self.assert_security {
            for sec in secs {
                if !SECURITY_TECHS.contains(&sec.as_str()) {
                    return Err(format!("Invalid AssertSecurity '{}': must be one of {:?}", sec, SECURITY_TECHS));
                }
            }
        }

        // Condition capabilities - these are like CAP_MKNOD, but validating all would be extensive
        // For now, just check they start with CAP_
        if let Some(ref caps) = self.condition_capability {
            for cap in caps {
                if !cap.starts_with("CAP_") {
                    return Err(format!("Invalid ConditionCapability '{}': must start with CAP_", cap));
                }
            }
        }
        if let Some(ref caps) = self.assert_capability {
            for cap in caps {
                if !cap.starts_with("CAP_") {
                    return Err(format!("Invalid AssertCapability '{}': must start with CAP_", cap));
                }
            }
        }

        Ok(())
    }
}
