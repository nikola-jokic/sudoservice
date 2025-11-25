use std::fmt;
use std::path::PathBuf;

/// Execution environment options
/// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#Options
#[derive(Debug, Clone, Default)]
pub struct Exec {
    // Path settings
    /// Working directory for executed processes.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#WorkingDirectory=
    pub working_directory: Option<PathBuf>,

    /// Root directory for executed processes.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RootDirectory=
    pub root_directory: Option<PathBuf>,

    /// Image to mount as root directory.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RootImage=
    pub root_image: Option<PathBuf>,

    /// Options for mounting the root image.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RootImageOptions=
    pub root_image_options: Option<Vec<String>>,

    /// Verity data for the root image.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RootVerity=
    pub root_verity: Option<PathBuf>,

    /// Hash for root verity.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RootHash=
    pub root_hash: Option<String>,

    /// Signature for root hash.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RootHashSignature=
    pub root_hash_signature: Option<PathBuf>,

    /// Path to cryptographic key for root hash signature.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RootHashSignature=
    pub root_hash_signature_key: Option<PathBuf>,

    /// Path to the PKCS#7 signature of the root hash.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RootHashSignature=
    pub root_hash_signature_pcr_private_key: Option<PathBuf>,

    /// Path to the PKCS#7 signature of the root hash.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RootHashSignature=
    pub root_hash_signature_pcr_public_key: Option<PathBuf>,

    /// Ephemeral root directory.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#RootEphemeral=
    pub root_ephemeral: Option<bool>,

    // User/Group settings
    /// User identity for executed processes.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#User=
    pub user: Option<String>,

    /// Group identity for executed processes.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#Group=
    pub group: Option<String>,

    /// Supplementary groups for executed processes.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#SupplementaryGroups=
    pub supplementary_groups: Option<Vec<String>>,

    /// PAM service name.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#PAMName=
    pub pam_name: Option<String>,

    // Environment settings
    /// Environment variables to set.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#Environment=
    pub environment: Option<Vec<String>>,

    /// Environment files to read.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#EnvironmentFile=
    pub environment_file: Option<Vec<PathBuf>>,

    /// Pass environment from parent.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#PassEnvironment=
    pub pass_environment: Option<Vec<String>>,

    /// Unset environment variables.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#UnsetEnvironment=
    pub unset_environment: Option<Vec<String>>,

    // Resource limits - these would be extensive, implementing just a few for now
    /// CPU time limit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#LimitCPU=
    pub limit_cpu: Option<String>,

    /// File size limit.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#LimitFSIZE=
    pub limit_fsize: Option<String>,

    // Security settings
    /// No new privileges.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#NoNewPrivileges=
    pub no_new_privileges: Option<bool>,

    /// Private temporary directory.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#PrivateTmp=
    pub private_tmp: Option<bool>,

    /// Private devices.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#PrivateDevices=
    pub private_devices: Option<bool>,

    /// Protect kernel tunables.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#ProtectKernelTunables=
    pub protect_kernel_tunables: Option<bool>,

    /// Protect kernel modules.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#ProtectKernelModules=
    pub protect_kernel_modules: Option<bool>,

    /// Protect control groups.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#ProtectControlGroups=
    pub protect_control_groups: Option<bool>,

    // Capability settings
    /// Capability bounding set.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#CapabilityBoundingSet=
    pub capability_bounding_set: Option<Vec<String>>,

    /// Ambient capabilities.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#AmbientCapabilities=
    pub ambient_capabilities: Option<Vec<String>>,

    // Process settings
    /// Nice level.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#Nice=
    pub nice: Option<i32>,

    /// OOM score adjust.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#OOMScoreAdjust=
    pub oom_score_adjust: Option<i32>,

    // File system settings
    /// Read-write paths.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#ReadWritePaths=
    pub read_write_paths: Option<Vec<PathBuf>>,

    /// Read-only paths.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#ReadOnlyPaths=
    pub read_only_paths: Option<Vec<PathBuf>>,

    /// Inaccessible paths.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#InaccessiblePaths=
    pub inaccessible_paths: Option<Vec<PathBuf>>,

    // Network settings
    /// Private network.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#PrivateNetwork=
    pub private_network: Option<bool>,

    /// Network namespace path.
    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html#NetworkNamespacePath=
    pub network_namespace_path: Option<PathBuf>,
}

impl fmt::Display for Exec {
    fn fmt(&self, buf: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Path settings
        if let Some(ref path) = self.working_directory {
            writeln!(buf, "WorkingDirectory={}", path.display())?;
        }
        if let Some(ref path) = self.root_directory {
            writeln!(buf, "RootDirectory={}", path.display())?;
        }
        if let Some(ref path) = self.root_image {
            writeln!(buf, "RootImage={}", path.display())?;
        }
        write_vec!(buf, self.root_image_options, "RootImageOptions");
        if let Some(ref path) = self.root_verity {
            writeln!(buf, "RootVerity={}", path.display())?;
        }
        write_option!(buf, self.root_hash, "RootHash");
        if let Some(ref path) = self.root_hash_signature {
            writeln!(buf, "RootHashSignature={}", path.display())?;
        }
        if let Some(ref path) = self.root_hash_signature_key {
            writeln!(buf, "RootHashSignature={}", path.display())?;
        }
        if let Some(ref path) = self.root_hash_signature_pcr_private_key {
            writeln!(buf, "RootHashSignature={}", path.display())?;
        }
        if let Some(ref path) = self.root_hash_signature_pcr_public_key {
            writeln!(buf, "RootHashSignature={}", path.display())?;
        }
        write_bool!(buf, self.root_ephemeral, "RootEphemeral");

        // User/Group settings
        write_option!(buf, self.user, "User");
        write_option!(buf, self.group, "Group");
        write_vec!(buf, self.supplementary_groups, "SupplementaryGroups");
        write_option!(buf, self.pam_name, "PAMName");

        // Environment settings
        write_vec!(buf, self.environment, "Environment");
        if let Some(ref files) = self.environment_file {
            for file in files {
                writeln!(buf, "EnvironmentFile={}", file.display())?;
            }
        }
        write_vec!(buf, self.pass_environment, "PassEnvironment");
        write_vec!(buf, self.unset_environment, "UnsetEnvironment");

        // Resource limits
        write_option!(buf, self.limit_cpu, "LimitCPU");
        write_option!(buf, self.limit_fsize, "LimitFSIZE");

        // Security settings
        write_bool!(buf, self.no_new_privileges, "NoNewPrivileges");
        write_bool!(buf, self.private_tmp, "PrivateTmp");
        write_bool!(buf, self.private_devices, "PrivateDevices");
        write_bool!(buf, self.protect_kernel_tunables, "ProtectKernelTunables");
        write_bool!(buf, self.protect_kernel_modules, "ProtectKernelModules");
        write_bool!(buf, self.protect_control_groups, "ProtectControlGroups");

        // Capability settings
        write_vec!(buf, self.capability_bounding_set, "CapabilityBoundingSet");
        write_vec!(buf, self.ambient_capabilities, "AmbientCapabilities");

        // Process settings
        if let Some(nice) = self.nice {
            writeln!(buf, "Nice={}", nice)?;
        }
        if let Some(score) = self.oom_score_adjust {
            writeln!(buf, "OOMScoreAdjust={}", score)?;
        }

        // File system settings
        if let Some(ref paths) = self.read_write_paths {
            for path in paths {
                writeln!(buf, "ReadWritePaths={}", path.display())?;
            }
        }
        if let Some(ref paths) = self.read_only_paths {
            for path in paths {
                writeln!(buf, "ReadOnlyPaths={}", path.display())?;
            }
        }
        if let Some(ref paths) = self.inaccessible_paths {
            for path in paths {
                writeln!(buf, "InaccessiblePaths={}", path.display())?;
            }
        }

        // Network settings
        write_bool!(buf, self.private_network, "PrivateNetwork");
        if let Some(ref path) = self.network_namespace_path {
            writeln!(buf, "NetworkNamespacePath={}", path.display())?;
        }

        Ok(())
    }
}

impl Exec {
    /// Creates a new empty `Exec` section.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the execution environment configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate nice level (-20 to 19)
        if let Some(nice) = self.nice {
            if nice < -20 || nice > 19 {
                return Err(format!("Nice level {} must be between -20 and 19", nice));
            }
        }

        // Validate OOM score adjust (-1000 to 1000)
        if let Some(score) = self.oom_score_adjust {
            if score < -1000 || score > 1000 {
                return Err(format!(
                    "OOMScoreAdjust {} must be between -1000 and 1000",
                    score
                ));
            }
        }

        // Validate environment variables (basic KEY=VALUE format)
        if let Some(ref env_vars) = self.environment {
            for env_var in env_vars {
                if !env_var.contains('=') {
                    return Err(format!(
                        "Environment variable '{}' must be in KEY=VALUE format",
                        env_var
                    ));
                }
                let parts: Vec<&str> = env_var.splitn(2, '=').collect();
                if parts.len() != 2 || parts[0].is_empty() {
                    return Err(format!(
                        "Invalid environment variable format: '{}'",
                        env_var
                    ));
                }
            }
        }

        // Validate pass environment variables (should not contain =)
        if let Some(ref pass_vars) = self.pass_environment {
            for var in pass_vars {
                if var.contains('=') {
                    return Err(format!(
                        "PassEnvironment variable '{}' should not contain '='",
                        var
                    ));
                }
            }
        }

        // Validate unset environment variables (should not contain =)
        if let Some(ref unset_vars) = self.unset_environment {
            for var in unset_vars {
                if var.contains('=') {
                    return Err(format!(
                        "UnsetEnvironment variable '{}' should not contain '='",
                        var
                    ));
                }
            }
        }

        // Validate capabilities (should start with CAP_)
        if let Some(ref caps) = self.capability_bounding_set {
            for cap in caps {
                if !cap.starts_with("CAP_") {
                    return Err(format!("Capability '{}' must start with 'CAP_'", cap));
                }
            }
        }

        if let Some(ref caps) = self.ambient_capabilities {
            for cap in caps {
                if !cap.starts_with("CAP_") {
                    return Err(format!("Capability '{}' must start with 'CAP_'", cap));
                }
            }
        }

        // Validate supplementary groups (should not be empty)
        if let Some(ref groups) = self.supplementary_groups {
            for group in groups {
                if group.is_empty() {
                    return Err("Supplementary group names cannot be empty".to_string());
                }
            }
        }

        // Validate root image options (basic format check)
        if let Some(ref options) = self.root_image_options {
            for option in options {
                if option.is_empty() {
                    return Err("RootImageOptions cannot contain empty strings".to_string());
                }
            }
        }

        Ok(())
    }

    // Builder pattern setters - implementing just a few key ones for now
    pub fn working_directory(mut self, value: PathBuf) -> Self {
        self.working_directory = Some(value);
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.group = Some(value.into());
        self
    }

    pub fn environment(mut self, value: Vec<impl Into<String>>) -> Self {
        self.environment = Some(value.into_iter().map(|s| s.into()).collect());
        self
    }

    pub fn no_new_privileges(mut self, value: bool) -> Self {
        self.no_new_privileges = Some(value);
        self
    }

    pub fn private_tmp(mut self, value: bool) -> Self {
        self.private_tmp = Some(value);
        self
    }
}
