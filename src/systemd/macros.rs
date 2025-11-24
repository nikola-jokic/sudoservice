/// Helper macros for formatting systemd unit file sections

/// Write an Option<String> field as "Key=value"
#[macro_export]
macro_rules! write_option {
    ($buf:expr, $field:expr, $key:expr) => {
        if let Some(ref value) = $field {
            writeln!($buf, "{}={}", $key, value)?;
        }
    };
}

/// Write an Option<Vec<T>> field with values space-separated on one line, where T: Display
#[macro_export]
macro_rules! write_vec {
    ($buf:expr, $field:expr, $key:expr) => {
        if let Some(ref values) = $field {
            if !values.is_empty() {
                writeln!(
                    $buf,
                    "{}={}",
                    $key,
                    values
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                )?;
            }
        }
    };
}

/// Write an Option<Vec<String>> field with one line per value (for multi-value options like ExecStart)
#[macro_export]
macro_rules! write_vec_multi {
    ($buf:expr, $field:expr, $key:expr) => {
        if let Some(ref values) = $field {
            for value in values {
                writeln!($buf, "{}={}", $key, value)?;
            }
        }
    };
}

/// Write an Option<bool> field as "Key=yes" or "Key=no"
#[macro_export]
macro_rules! write_bool {
    ($buf:expr, $field:expr, $key:expr) => {
        if let Some(value) = $field {
            writeln!($buf, "{}={}", $key, if value { "yes" } else { "no" })?;
        }
    };
}
