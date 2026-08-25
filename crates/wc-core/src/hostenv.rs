use std::env;

pub fn detect_shell(setting: Option<&str>) -> String {
    resolve_shell(
        setting,
        std::env::consts::OS,
        env::var("SHELL").ok().as_deref(),
        env::var("PSModulePath").ok().as_deref(),
        env::var("POWERSHELL_DISTRIBUTION_CHANNEL").ok().as_deref(),
    )
}

pub fn resolve_shell(
    setting: Option<&str>,
    os: &str,
    shell_env: Option<&str>,
    ps_module_path: Option<&str>,
    ps_distribution_channel: Option<&str>,
) -> String {
    if let Some(s) = setting {
        let s = s.trim();
        if !s.is_empty() && !s.eq_ignore_ascii_case("auto") {
            return s.to_ascii_lowercase();
        }
    }
    if os == "windows" {
        if ps_distribution_channel.is_some() {
            return "pwsh".into();
        }
        if ps_module_path.is_some_and(|p| p.to_ascii_lowercase().contains("powershell")) {
            return "powershell".into();
        }
        "cmd".into()
    } else {
        shell_env
            .map(|s| {
                s.trim()
                    .rsplit(['/', '\\'])
                    .next()
                    .unwrap_or("")
                    .trim_end_matches(".exe")
                    .to_string()
            })
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "bash".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_setting_wins_over_detection() {
        assert_eq!(
            resolve_shell(
                Some("pwsh"),
                "windows",
                None,
                Some("C:\\PS"),
                Some("channel")
            ),
            "pwsh"
        );
        assert_eq!(
            resolve_shell(Some("fish"), "linux", Some("/usr/bin/zsh"), None, None),
            "fish"
        );
    }

    #[test]
    fn auto_setting_falls_through_to_detection() {
        assert_eq!(
            resolve_shell(
                Some("auto"),
                "windows",
                None,
                Some("C:\\Windows\\System32\\WindowsPowerShell\\1.0\\Modules\\"),
                None
            ),
            "powershell"
        );
    }

    #[test]
    fn windows_detects_pwsh_via_distribution_channel() {
        assert_eq!(
            resolve_shell(None, "windows", None, None, Some("msix")),
            "pwsh"
        );
    }

    #[test]
    fn windows_detects_powershell_via_module_path() {
        assert_eq!(
            resolve_shell(
                None,
                "windows",
                None,
                Some("C:\\Windows\\System32\\WindowsPowerShell\\1.0\\"),
                None
            ),
            "powershell"
        );
        assert_eq!(
            resolve_shell(
                None,
                "windows",
                None,
                Some("c:\\program files\\powershell\\7"),
                None
            ),
            "powershell"
        );
    }

    #[test]
    fn windows_defaults_to_cmd_without_powershell_markers() {
        assert_eq!(resolve_shell(None, "windows", None, None, None), "cmd");
    }

    #[test]
    fn unix_uses_shell_basename_without_exe_suffix() {
        assert_eq!(
            resolve_shell(None, "linux", Some("/bin/zsh"), None, None),
            "zsh"
        );
        assert_eq!(
            resolve_shell(None, "macos", Some("/opt/homebrew/bin/fish"), None, None),
            "fish"
        );
        assert_eq!(
            resolve_shell(None, "freebsd", Some("pwsh.exe"), None, None),
            "pwsh"
        );
    }

    #[test]
    fn unix_defaults_to_bash_when_shell_unset() {
        assert_eq!(resolve_shell(None, "linux", None, None, None), "bash");
        assert_eq!(resolve_shell(None, "linux", Some("  "), None, None), "bash");
    }
}
