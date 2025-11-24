use std::fmt;
use std::path::PathBuf;

/// Collect mode for unit garbage collection
#[derive(Debug, Clone, Copy)]
pub enum CollectMode {
    /// Only collect inactive units
    Inactive,
    /// Collect inactive or failed units
    InactiveOrFailed,
}

impl fmt::Display for CollectMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectMode::Inactive => write!(f, "inactive"),
            CollectMode::InactiveOrFailed => write!(f, "inactive-or-failed"),
        }
    }
}

/// Job mode for OnSuccess/OnFailure units
#[derive(Debug, Clone, Copy)]
pub enum JobMode {
    /// Fail the job
    Fail,
    /// Replace the job
    Replace,
    /// Replace the job irreversibly
    ReplaceIrreversibly,
    /// Isolate the job
    Isolate,
    /// Flush the job
    Flush,
    /// Ignore dependencies
    IgnoreDependencies,
    /// Ignore requirements
    IgnoreRequirements,
}

impl fmt::Display for JobMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobMode::Fail => write!(f, "fail"),
            JobMode::Replace => write!(f, "replace"),
            JobMode::ReplaceIrreversibly => write!(f, "replace-irreversibly"),
            JobMode::Isolate => write!(f, "isolate"),
            JobMode::Flush => write!(f, "flush"),
            JobMode::IgnoreDependencies => write!(f, "ignore-dependencies"),
            JobMode::IgnoreRequirements => write!(f, "ignore-requirements"),
        }
    }
}

/// Action for failure/success/timeout scenarios
#[derive(Debug, Clone, Copy)]
pub enum Action {
    /// No action
    None,
    /// Reboot the system
    Reboot,
    /// Force reboot the system
    RebootForce,
    /// Reboot the system immediately
    RebootImmediate,
    /// Power off the system
    Poweroff,
    /// Force power off the system
    PoweroffForce,
    /// Power off the system immediately
    PoweroffImmediate,
    /// Exit the service
    Exit,
    /// Force exit the service
    ExitForce,
    /// Soft reboot the system
    SoftReboot,
    /// Force soft reboot the system
    SoftRebootForce,
    /// Use kexec to reboot
    Kexec,
    /// Force kexec to reboot
    KexecForce,
    /// Halt the system
    Halt,
    /// Force halt the system
    HaltForce,
    /// Halt the system immediately
    HaltImmediate,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::None => write!(f, "none"),
            Action::Reboot => write!(f, "reboot"),
            Action::RebootForce => write!(f, "reboot-force"),
            Action::RebootImmediate => write!(f, "reboot-immediate"),
            Action::Poweroff => write!(f, "poweroff"),
            Action::PoweroffForce => write!(f, "poweroff-force"),
            Action::PoweroffImmediate => write!(f, "poweroff-immediate"),
            Action::Exit => write!(f, "exit"),
            Action::ExitForce => write!(f, "exit-force"),
            Action::SoftReboot => write!(f, "soft-reboot"),
            Action::SoftRebootForce => write!(f, "soft-reboot-force"),
            Action::Kexec => write!(f, "kexec"),
            Action::KexecForce => write!(f, "kexec-force"),
            Action::Halt => write!(f, "halt"),
            Action::HaltForce => write!(f, "halt-force"),
            Action::HaltImmediate => write!(f, "halt-immediate"),
        }
    }
}

/// Security technology for condition/assert checks
#[derive(Debug, Clone, Copy)]
pub enum SecurityTech {
    /// SELinux security module
    Selinux,
    /// AppArmor security module
    Apparmor,
    /// TOMOYO security module
    Tomoyo,
    /// SMACK security module
    Smack,
    /// Integrity Measurement Architecture
    Ima,
    /// Linux Audit subsystem
    Audit,
    /// UEFI Secure Boot
    UefiSecureboot,
    /// TPM 2.0
    Tpm2,
    /// Confidential Virtual Machine
    Cvm,
    /// Measured Unified Kernel Image
    MeasuredUki,
}

impl fmt::Display for SecurityTech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityTech::Selinux => write!(f, "selinux"),
            SecurityTech::Apparmor => write!(f, "apparmor"),
            SecurityTech::Tomoyo => write!(f, "tomoyo"),
            SecurityTech::Smack => write!(f, "smack"),
            SecurityTech::Ima => write!(f, "ima"),
            SecurityTech::Audit => write!(f, "audit"),
            SecurityTech::UefiSecureboot => write!(f, "uefi-secureboot"),
            SecurityTech::Tpm2 => write!(f, "tpm2"),
            SecurityTech::Cvm => write!(f, "cvm"),
            SecurityTech::MeasuredUki => write!(f, "measured-uki"),
        }
    }
}

/// Architecture for condition/assert checks
#[derive(Debug, Clone, Copy)]
pub enum Architecture {
    /// x86 architecture
    X86,
    /// x86-64 architecture
    X8664,
    /// PowerPC architecture
    Ppc,
    /// PowerPC little-endian
    PpcLe,
    /// PowerPC 64-bit
    Ppc64,
    /// PowerPC 64-bit little-endian
    Ppc64Le,
    /// IA-64 architecture
    Ia64,
    /// PA-RISC architecture
    Parisc,
    /// PA-RISC 64-bit
    Parisc64,
    /// s390 architecture
    S390,
    /// s390x architecture
    S390x,
    /// SPARC architecture
    Sparc,
    /// SPARC 64-bit
    Sparc64,
    /// MIPS architecture
    Mips,
    /// MIPS little-endian
    MipsLe,
    /// MIPS 64-bit
    Mips64,
    /// MIPS 64-bit little-endian
    Mips64Le,
    /// Alpha architecture
    Alpha,
    /// ARM architecture
    Arm,
    /// ARM big-endian
    ArmBe,
    /// ARM 64-bit
    Arm64,
    /// ARM 64-bit big-endian
    Arm64Be,
    /// SuperH architecture
    Sh,
    /// SuperH 64-bit
    Sh64,
    /// Motorola 68000
    M68k,
    /// Tilera TILE-Gx
    Tilegx,
    /// CRIS architecture
    Cris,
    /// ARC architecture
    Arc,
    /// ARC big-endian
    ArcBe,
    /// Native architecture
    Native,
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::X86 => write!(f, "x86"),
            Architecture::X8664 => write!(f, "x86-64"),
            Architecture::Ppc => write!(f, "ppc"),
            Architecture::PpcLe => write!(f, "ppc-le"),
            Architecture::Ppc64 => write!(f, "ppc64"),
            Architecture::Ppc64Le => write!(f, "ppc64-le"),
            Architecture::Ia64 => write!(f, "ia64"),
            Architecture::Parisc => write!(f, "parisc"),
            Architecture::Parisc64 => write!(f, "parisc64"),
            Architecture::S390 => write!(f, "s390"),
            Architecture::S390x => write!(f, "s390x"),
            Architecture::Sparc => write!(f, "sparc"),
            Architecture::Sparc64 => write!(f, "sparc64"),
            Architecture::Mips => write!(f, "mips"),
            Architecture::MipsLe => write!(f, "mips-le"),
            Architecture::Mips64 => write!(f, "mips64"),
            Architecture::Mips64Le => write!(f, "mips64-le"),
            Architecture::Alpha => write!(f, "alpha"),
            Architecture::Arm => write!(f, "arm"),
            Architecture::ArmBe => write!(f, "arm-be"),
            Architecture::Arm64 => write!(f, "arm64"),
            Architecture::Arm64Be => write!(f, "arm64-be"),
            Architecture::Sh => write!(f, "sh"),
            Architecture::Sh64 => write!(f, "sh64"),
            Architecture::M68k => write!(f, "m68k"),
            Architecture::Tilegx => write!(f, "tilegx"),
            Architecture::Cris => write!(f, "cris"),
            Architecture::Arc => write!(f, "arc"),
            Architecture::ArcBe => write!(f, "arc-be"),
            Architecture::Native => write!(f, "native"),
        }
    }
}

/// Virtualization type for condition/assert checks
#[derive(Debug, Clone, Copy)]
pub enum Virtualization {
    /// Generic virtual machine
    Vm,
    /// Container
    Container,
    /// QEMU
    Qemu,
    /// KVM
    Kvm,
    /// Amazon EC2
    Amazon,
    /// z/VM
    Zvm,
    /// VMware
    Vmware,
    /// Microsoft Hyper-V
    Microsoft,
    /// Oracle VirtualBox
    Oracle,
    /// IBM PowerVM
    Powervm,
    /// Xen
    Xen,
    /// Bochs
    Bochs,
    /// User-mode Linux
    Uml,
    /// bhyve
    Bhyve,
    /// QNX
    Qnx,
    /// Apple Virtualization.framework
    Apple,
    /// System Runtime Environment
    Sre,
    /// OpenVZ
    Openvz,
    /// LXC
    Lxc,
    /// LXC via libvirt
    LxcLibvirt,
    /// systemd-nspawn
    SystemdNspawn,
    /// Docker
    Docker,
    /// Podman
    Podman,
    /// rkt
    Rkt,
    /// Windows Subsystem for Linux
    Wsl,
    /// PRoot
    Proot,
    /// Pouch
    Pouch,
    /// ACRN
    Acrn,
    /// Private users
    PrivateUsers,
}

impl fmt::Display for Virtualization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Virtualization::Vm => write!(f, "vm"),
            Virtualization::Container => write!(f, "container"),
            Virtualization::Qemu => write!(f, "qemu"),
            Virtualization::Kvm => write!(f, "kvm"),
            Virtualization::Amazon => write!(f, "amazon"),
            Virtualization::Zvm => write!(f, "zvm"),
            Virtualization::Vmware => write!(f, "vmware"),
            Virtualization::Microsoft => write!(f, "microsoft"),
            Virtualization::Oracle => write!(f, "oracle"),
            Virtualization::Powervm => write!(f, "powervm"),
            Virtualization::Xen => write!(f, "xen"),
            Virtualization::Bochs => write!(f, "bochs"),
            Virtualization::Uml => write!(f, "uml"),
            Virtualization::Bhyve => write!(f, "bhyve"),
            Virtualization::Qnx => write!(f, "qnx"),
            Virtualization::Apple => write!(f, "apple"),
            Virtualization::Sre => write!(f, "sre"),
            Virtualization::Openvz => write!(f, "openvz"),
            Virtualization::Lxc => write!(f, "lxc"),
            Virtualization::LxcLibvirt => write!(f, "lxc-libvirt"),
            Virtualization::SystemdNspawn => write!(f, "systemd-nspawn"),
            Virtualization::Docker => write!(f, "docker"),
            Virtualization::Podman => write!(f, "podman"),
            Virtualization::Rkt => write!(f, "rkt"),
            Virtualization::Wsl => write!(f, "wsl"),
            Virtualization::Proot => write!(f, "proot"),
            Virtualization::Pouch => write!(f, "pouch"),
            Virtualization::Acrn => write!(f, "acrn"),
            Virtualization::PrivateUsers => write!(f, "private-users"),
        }
    }
}

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
    pub on_success_job_mode: Option<JobMode>,

    /// Job mode for OnFailure= units.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#OnFailureJobMode=
    pub on_failure_job_mode: Option<JobMode>,

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
    pub collect_mode: Option<CollectMode>,

    /// Action to take when the unit stops and enters a failed state.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#FailureAction=
    pub failure_action: Option<Action>,

    /// Action to take when the unit enters an inactive state.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#SuccessAction=
    pub success_action: Option<Action>,

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
    pub job_timeout_action: Option<Action>,

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
    pub start_limit_action: Option<Action>,

    /// Argument for the reboot(2) system call if StartLimitAction= is a reboot action.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#RebootArgument=
    pub reboot_argument: Option<String>,

    /// Path to a configuration file this unit has been generated from.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#SourcePath=
    pub source_path: Option<PathBuf>,

    // Condition checks
    /// Check whether the system is running on a specific architecture.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionArchitecture=
    pub condition_architecture: Option<Architecture>,

    /// Check whether the system's firmware is of a certain type.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionFirmware=
    pub condition_firmware: Option<String>,

    /// Check whether the system is executed in a virtualized environment.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#ConditionVirtualization=
    pub condition_virtualization: Option<Virtualization>,

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
    pub condition_security: Option<Vec<SecurityTech>>,

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
    pub assert_architecture: Option<Architecture>,

    /// Assert that the system's firmware is of a certain type.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertFirmware=
    pub assert_firmware: Option<String>,

    /// Assert that the system is executed in a virtualized environment.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#AssertVirtualization=
    pub assert_virtualization: Option<Virtualization>,

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
    pub assert_security: Option<Vec<SecurityTech>>,

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

impl Unit {
    /// Creates a new empty `Unit` section.
    pub fn new() -> Self {
        Self::default()
    }
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
                    && !doc.starts_with("man:")
                {
                    return Err(format!(
                        "Invalid documentation URI '{}': must start with http://, https://, file:, info:, or man:",
                        doc
                    ));
                }
            }
        }

        // Condition firmware
        // This is more complex, but for now skip detailed validation

        // Condition capabilities - these are like CAP_MKNOD, but validating all would be extensive
        // For now, just check they start with CAP_
        if let Some(ref caps) = self.condition_capability {
            for cap in caps {
                if !cap.starts_with("CAP_") {
                    return Err(format!(
                        "Invalid ConditionCapability '{}': must start with CAP_",
                        cap
                    ));
                }
            }
        }
        if let Some(ref caps) = self.assert_capability {
            for cap in caps {
                if !cap.starts_with("CAP_") {
                    return Err(format!(
                        "Invalid AssertCapability '{}': must start with CAP_",
                        cap
                    ));
                }
            }
        }

        Ok(())
    }

    // Builder pattern setters
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn documentation(mut self, value: Vec<impl Into<String>>) -> Self {
        self.documentation = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn wants(mut self, value: Vec<impl Into<String>>) -> Self {
        self.wants = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn requires(mut self, value: Vec<impl Into<String>>) -> Self {
        self.requires = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn requisite(mut self, value: Vec<impl Into<String>>) -> Self {
        self.requisite = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn binds_to(mut self, value: Vec<impl Into<String>>) -> Self {
        self.binds_to = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn part_of(mut self, value: Vec<impl Into<String>>) -> Self {
        self.part_of = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn upholds(mut self, value: Vec<impl Into<String>>) -> Self {
        self.upholds = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn conflicts(mut self, value: Vec<impl Into<String>>) -> Self {
        self.conflicts = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn before(mut self, value: Vec<impl Into<String>>) -> Self {
        self.before = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn after(mut self, value: Vec<impl Into<String>>) -> Self {
        self.after = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn on_failure(mut self, value: Vec<impl Into<String>>) -> Self {
        self.on_failure = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn on_success(mut self, value: Vec<impl Into<String>>) -> Self {
        self.on_success = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn propagates_reload_to(mut self, value: Vec<impl Into<String>>) -> Self {
        self.propagates_reload_to = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn reload_propagated_from(mut self, value: Vec<impl Into<String>>) -> Self {
        self.reload_propagated_from = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn propagates_stop_to(mut self, value: Vec<impl Into<String>>) -> Self {
        self.propagates_stop_to = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn stop_propagated_from(mut self, value: Vec<impl Into<String>>) -> Self {
        self.stop_propagated_from = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn joins_namespace_of(mut self, value: Vec<impl Into<String>>) -> Self {
        self.joins_namespace_of = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn requires_mounts_for(mut self, value: Vec<impl Into<String>>) -> Self {
        self.requires_mounts_for = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn wants_mounts_for(mut self, value: Vec<impl Into<String>>) -> Self {
        self.wants_mounts_for = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn on_success_job_mode(mut self, value: JobMode) -> Self {
        self.on_success_job_mode = Some(value);
        self
    }

    pub fn on_failure_job_mode(mut self, value: JobMode) -> Self {
        self.on_failure_job_mode = Some(value);
        self
    }

    pub fn ignore_on_isolate(mut self, value: bool) -> Self {
        self.ignore_on_isolate = Some(value);
        self
    }

    pub fn stop_when_unneeded(mut self, value: bool) -> Self {
        self.stop_when_unneeded = Some(value);
        self
    }

    pub fn refuse_manual_start(mut self, value: bool) -> Self {
        self.refuse_manual_start = Some(value);
        self
    }

    pub fn refuse_manual_stop(mut self, value: bool) -> Self {
        self.refuse_manual_stop = Some(value);
        self
    }

    pub fn allow_isolate(mut self, value: bool) -> Self {
        self.allow_isolate = Some(value);
        self
    }

    pub fn default_dependencies(mut self, value: bool) -> Self {
        self.default_dependencies = Some(value);
        self
    }

    pub fn survive_final_kill_signal(mut self, value: bool) -> Self {
        self.survive_final_kill_signal = Some(value);
        self
    }

    pub fn collect_mode(mut self, value: CollectMode) -> Self {
        self.collect_mode = Some(value);
        self
    }

    pub fn failure_action(mut self, value: Action) -> Self {
        self.failure_action = Some(value);
        self
    }

    pub fn success_action(mut self, value: Action) -> Self {
        self.success_action = Some(value);
        self
    }

    pub fn failure_action_exit_status(mut self, value: u8) -> Self {
        self.failure_action_exit_status = Some(value);
        self
    }

    pub fn success_action_exit_status(mut self, value: u8) -> Self {
        self.success_action_exit_status = Some(value);
        self
    }

    pub fn job_timeout_sec(mut self, value: u64) -> Self {
        self.job_timeout_sec = Some(value);
        self
    }

    pub fn job_running_timeout_sec(mut self, value: u64) -> Self {
        self.job_running_timeout_sec = Some(value);
        self
    }

    pub fn job_timeout_action(mut self, value: Action) -> Self {
        self.job_timeout_action = Some(value);
        self
    }

    pub fn job_timeout_reboot_argument(mut self, value: impl Into<String>) -> Self {
        self.job_timeout_reboot_argument = Some(value.into());
        self
    }

    pub fn start_limit_interval_sec(mut self, value: u64) -> Self {
        self.start_limit_interval_sec = Some(value);
        self
    }

    pub fn start_limit_burst(mut self, value: u32) -> Self {
        self.start_limit_burst = Some(value);
        self
    }

    pub fn start_limit_action(mut self, value: Action) -> Self {
        self.start_limit_action = Some(value);
        self
    }

    pub fn reboot_argument(mut self, value: impl Into<String>) -> Self {
        self.reboot_argument = Some(value.into());
        self
    }

    pub fn source_path(mut self, value: PathBuf) -> Self {
        self.source_path = Some(value);
        self
    }

    pub fn condition_architecture(mut self, value: Architecture) -> Self {
        self.condition_architecture = Some(value);
        self
    }

    pub fn condition_firmware(mut self, value: impl Into<String>) -> Self {
        self.condition_firmware = Some(value.into());
        self
    }

    pub fn condition_virtualization(mut self, value: Virtualization) -> Self {
        self.condition_virtualization = Some(value);
        self
    }

    pub fn condition_host(mut self, value: impl Into<String>) -> Self {
        self.condition_host = Some(value.into());
        self
    }

    pub fn condition_kernel_command_line(mut self, value: impl Into<String>) -> Self {
        self.condition_kernel_command_line = Some(value.into());
        self
    }

    pub fn condition_kernel_version(mut self, value: impl Into<String>) -> Self {
        self.condition_kernel_version = Some(value.into());
        self
    }

    pub fn condition_credential(mut self, value: impl Into<String>) -> Self {
        self.condition_credential = Some(value.into());
        self
    }

    pub fn condition_environment(mut self, value: impl Into<String>) -> Self {
        self.condition_environment = Some(value.into());
        self
    }

    pub fn condition_security(mut self, value: Vec<SecurityTech>) -> Self {
        self.condition_security = Some(value);
        self
    }

    pub fn condition_capability(mut self, value: Vec<impl Into<String>>) -> Self {
        self.condition_capability = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn condition_ac_power(mut self, value: bool) -> Self {
        self.condition_ac_power = Some(value);
        self
    }

    pub fn condition_needs_update(mut self, value: impl Into<String>) -> Self {
        self.condition_needs_update = Some(value.into());
        self
    }

    pub fn condition_first_boot(mut self, value: bool) -> Self {
        self.condition_first_boot = Some(value);
        self
    }

    pub fn condition_path_exists(mut self, value: impl Into<String>) -> Self {
        self.condition_path_exists = Some(value.into());
        self
    }

    pub fn condition_path_exists_glob(mut self, value: impl Into<String>) -> Self {
        self.condition_path_exists_glob = Some(value.into());
        self
    }

    pub fn condition_path_is_directory(mut self, value: impl Into<String>) -> Self {
        self.condition_path_is_directory = Some(value.into());
        self
    }

    pub fn condition_path_is_symbolic_link(mut self, value: impl Into<String>) -> Self {
        self.condition_path_is_symbolic_link = Some(value.into());
        self
    }

    pub fn condition_path_is_mount_point(mut self, value: impl Into<String>) -> Self {
        self.condition_path_is_mount_point = Some(value.into());
        self
    }

    pub fn condition_path_is_read_write(mut self, value: impl Into<String>) -> Self {
        self.condition_path_is_read_write = Some(value.into());
        self
    }

    pub fn condition_path_is_encrypted(mut self, value: impl Into<String>) -> Self {
        self.condition_path_is_encrypted = Some(value.into());
        self
    }

    pub fn condition_directory_not_empty(mut self, value: impl Into<String>) -> Self {
        self.condition_directory_not_empty = Some(value.into());
        self
    }

    pub fn condition_file_not_empty(mut self, value: impl Into<String>) -> Self {
        self.condition_file_not_empty = Some(value.into());
        self
    }

    pub fn condition_file_is_executable(mut self, value: impl Into<String>) -> Self {
        self.condition_file_is_executable = Some(value.into());
        self
    }

    pub fn condition_user(mut self, value: impl Into<String>) -> Self {
        self.condition_user = Some(value.into());
        self
    }

    pub fn condition_group(mut self, value: impl Into<String>) -> Self {
        self.condition_group = Some(value.into());
        self
    }

    pub fn condition_control_group_controller(mut self, value: Vec<impl Into<String>>) -> Self {
        self.condition_control_group_controller =
            Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn condition_memory(mut self, value: impl Into<String>) -> Self {
        self.condition_memory = Some(value.into());
        self
    }

    pub fn condition_cpus(mut self, value: impl Into<String>) -> Self {
        self.condition_cpus = Some(value.into());
        self
    }

    pub fn condition_cpu_feature(mut self, value: Vec<impl Into<String>>) -> Self {
        self.condition_cpu_feature = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn condition_os_release(mut self, value: impl Into<String>) -> Self {
        self.condition_os_release = Some(value.into());
        self
    }

    pub fn condition_memory_pressure(mut self, value: impl Into<String>) -> Self {
        self.condition_memory_pressure = Some(value.into());
        self
    }

    pub fn condition_cpu_pressure(mut self, value: impl Into<String>) -> Self {
        self.condition_cpu_pressure = Some(value.into());
        self
    }

    pub fn condition_io_pressure(mut self, value: impl Into<String>) -> Self {
        self.condition_io_pressure = Some(value.into());
        self
    }

    pub fn assert_architecture(mut self, value: Architecture) -> Self {
        self.assert_architecture = Some(value);
        self
    }

    pub fn assert_firmware(mut self, value: impl Into<String>) -> Self {
        self.assert_firmware = Some(value.into());
        self
    }

    pub fn assert_virtualization(mut self, value: Virtualization) -> Self {
        self.assert_virtualization = Some(value);
        self
    }

    pub fn assert_host(mut self, value: impl Into<String>) -> Self {
        self.assert_host = Some(value.into());
        self
    }

    pub fn assert_kernel_command_line(mut self, value: impl Into<String>) -> Self {
        self.assert_kernel_command_line = Some(value.into());
        self
    }

    pub fn assert_kernel_version(mut self, value: impl Into<String>) -> Self {
        self.assert_kernel_version = Some(value.into());
        self
    }

    pub fn assert_credential(mut self, value: impl Into<String>) -> Self {
        self.assert_credential = Some(value.into());
        self
    }

    pub fn assert_environment(mut self, value: impl Into<String>) -> Self {
        self.assert_environment = Some(value.into());
        self
    }

    pub fn assert_security(mut self, value: Vec<SecurityTech>) -> Self {
        self.assert_security = Some(value);
        self
    }

    pub fn assert_capability(mut self, value: Vec<impl Into<String>>) -> Self {
        self.assert_capability = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn assert_ac_power(mut self, value: bool) -> Self {
        self.assert_ac_power = Some(value);
        self
    }

    pub fn assert_needs_update(mut self, value: impl Into<String>) -> Self {
        self.assert_needs_update = Some(value.into());
        self
    }

    pub fn assert_first_boot(mut self, value: bool) -> Self {
        self.assert_first_boot = Some(value);
        self
    }

    pub fn assert_path_exists(mut self, value: impl Into<String>) -> Self {
        self.assert_path_exists = Some(value.into());
        self
    }

    pub fn assert_path_exists_glob(mut self, value: impl Into<String>) -> Self {
        self.assert_path_exists_glob = Some(value.into());
        self
    }

    pub fn assert_path_is_directory(mut self, value: impl Into<String>) -> Self {
        self.assert_path_is_directory = Some(value.into());
        self
    }

    pub fn assert_path_is_symbolic_link(mut self, value: impl Into<String>) -> Self {
        self.assert_path_is_symbolic_link = Some(value.into());
        self
    }

    pub fn assert_path_is_mount_point(mut self, value: impl Into<String>) -> Self {
        self.assert_path_is_mount_point = Some(value.into());
        self
    }

    pub fn assert_path_is_read_write(mut self, value: impl Into<String>) -> Self {
        self.assert_path_is_read_write = Some(value.into());
        self
    }

    pub fn assert_path_is_encrypted(mut self, value: impl Into<String>) -> Self {
        self.assert_path_is_encrypted = Some(value.into());
        self
    }

    pub fn assert_directory_not_empty(mut self, value: impl Into<String>) -> Self {
        self.assert_directory_not_empty = Some(value.into());
        self
    }

    pub fn assert_file_not_empty(mut self, value: impl Into<String>) -> Self {
        self.assert_file_not_empty = Some(value.into());
        self
    }

    pub fn assert_file_is_executable(mut self, value: impl Into<String>) -> Self {
        self.assert_file_is_executable = Some(value.into());
        self
    }

    pub fn assert_user(mut self, value: impl Into<String>) -> Self {
        self.assert_user = Some(value.into());
        self
    }

    pub fn assert_group(mut self, value: impl Into<String>) -> Self {
        self.assert_group = Some(value.into());
        self
    }

    pub fn assert_control_group_controller(mut self, value: Vec<impl Into<String>>) -> Self {
        self.assert_control_group_controller = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn assert_memory(mut self, value: impl Into<String>) -> Self {
        self.assert_memory = Some(value.into());
        self
    }

    pub fn assert_cpus(mut self, value: impl Into<String>) -> Self {
        self.assert_cpus = Some(value.into());
        self
    }

    pub fn assert_cpu_feature(mut self, value: Vec<impl Into<String>>) -> Self {
        self.assert_cpu_feature = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn assert_os_release(mut self, value: impl Into<String>) -> Self {
        self.assert_os_release = Some(value.into());
        self
    }

    pub fn assert_memory_pressure(mut self, value: impl Into<String>) -> Self {
        self.assert_memory_pressure = Some(value.into());
        self
    }

    pub fn assert_cpu_pressure(mut self, value: impl Into<String>) -> Self {
        self.assert_cpu_pressure = Some(value.into());
        self
    }

    pub fn assert_io_pressure(mut self, value: impl Into<String>) -> Self {
        self.assert_io_pressure = Some(value.into());
        self
    }
}
