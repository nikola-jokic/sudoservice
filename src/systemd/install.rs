use std::fmt;

/// Represents the [Install] section of a systemd unit file.
///
/// This section carries installation information for the unit and is used
/// by systemctl enable/disable commands during installation of a unit.
#[derive(Debug, Clone, Default)]
pub struct Install {
    /// A space-separated list of additional names this unit shall be installed under.
    /// The names listed here must have the same suffix (i.e. type) as the unit filename.
    pub alias: Option<Vec<String>>,

    /// This has the effect of a dependency of type Wants= being added from the listed unit to the current unit.
    pub wanted_by: Option<Vec<String>>,

    /// This has the effect of a dependency of type Requires= being added from the listed unit to the current unit.
    pub required_by: Option<Vec<String>>,

    /// This has the effect of a dependency of type Upholds= being added from the listed unit to the current unit.
    pub upheld_by: Option<Vec<String>>,

    /// Additional units to install/deinstall when this unit is installed/deinstalled.
    pub also: Option<Vec<String>>,

    /// In template unit files, this specifies for which instance the unit shall be enabled
    /// if the template is enabled without any explicitly set instance.
    pub default_instance: Option<String>,
}

impl fmt::Display for Install {
    fn fmt(&self, buf: &mut fmt::Formatter<'_>) -> fmt::Result {
        buf.write_str("[Install]\n")?;

        write_vec!(buf, self.alias, "Alias");
        write_vec!(buf, self.wanted_by, "WantedBy");
        write_vec!(buf, self.required_by, "RequiredBy");
        write_vec!(buf, self.upheld_by, "UpheldBy");
        write_vec!(buf, self.also, "Also");
        write_option!(buf, self.default_instance, "DefaultInstance");
        Ok(())
    }
}
