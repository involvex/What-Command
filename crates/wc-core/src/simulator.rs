use std::collections::HashMap;

use crate::models::SimulateResult;

pub fn simulate_command(cmd: &str, vars: &HashMap<String, String>) -> SimulateResult {
    let mut resolved = cmd.trim().to_string();
    for (k, v) in vars {
        resolved = resolved.replace(&format!("{{{{{k}}}}}"), v);
        resolved = resolved.replace(&format!("${k}"), v);
    }
    let danger = estimate_danger(&resolved);
    simulate_command_inner(&resolved, danger)
}

pub fn simulate_command_inner(resolved: &str, danger: u8) -> SimulateResult {
    if danger >= 2 {
        return SimulateResult {
            explanation: "This command is flagged as destructive. Simulation is blocked for safety."
                .into(),
            sample_output: String::new(),
            exit_code: 1,
            blocked: true,
        };
    }

    let lower = resolved.to_lowercase();
    let (explanation, sample_output, exit_code) = if lower.starts_with("ls") {
        (
            "Lists directory contents (simulated).",
            "total 12\ndrwxr-xr-x  5 user  staff  160 Jun 17 09:00 .\ndrwxr-xr-x  8 user  staff  256 Jun 17 08:00 ..\n-rw-r--r--  1 user  staff  220 Jun 17 08:00 README.md",
            0,
        )
    } else if lower.starts_with("git status") {
        (
            "Shows working tree status (simulated).",
            "On branch main\nnothing to commit, working tree clean",
            0,
        )
    } else if lower.starts_with("echo ") {
        (
            "Prints text to stdout (simulated).",
            &resolved[5..],
            0,
        )
    } else if lower.starts_with("pwd") {
        (
            "Prints working directory (simulated).",
            "/home/user/project",
            0,
        )
    } else if lower.starts_with("find ") {
        (
            "Search for files (simulated).",
            "./src/main.rs\n./src/lib.rs",
            0,
        )
    } else {
        (
            "Preview only — no real shell execution.",
            "(simulated output)",
            0,
        )
    };

    SimulateResult {
        explanation: explanation.into(),
        sample_output: sample_output.to_string(),
        exit_code,
        blocked: false,
    }
}

fn estimate_danger(cmd: &str) -> u8 {
    let lower = cmd.to_lowercase();
    if lower.contains("rm -rf") || lower.contains("mkfs") || lower.contains(":(){") {
        3
    } else if lower.contains("rm ") || lower.contains("chmod") {
        2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn blocks_destructive() {
        let r = simulate_command("rm -rf /", &HashMap::new());
        assert!(r.blocked);
    }

    #[test]
    fn simulates_ls() {
        let r = simulate_command("ls -la", &HashMap::new());
        assert!(!r.blocked);
        assert!(r.sample_output.contains("total"));
    }
}
