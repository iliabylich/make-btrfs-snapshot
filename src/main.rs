mod spawn;

use miette::{Context, Result};
use owo_colors::OwoColorize;
use spawn::spawn_and_forward_stdout_and_stderr;

const DESCRIPTION_PREFIX: &str = "by make-btrfs-snapshot";
const KEEP_ONLY_SNAPSHOTS: usize = 5;

fn main() -> Result<()> {
    eprintln!("{}", "Starting make-btrfs-snapshot".green());

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    spawn_and_forward_stdout_and_stderr(
        "timeshift",
        [
            "--create".to_string(),
            "--comments".to_string(),
            format!("by make-btrfs-snapshot at {now}"),
        ],
    )?;

    let lines = spawn_and_forward_stdout_and_stderr("timeshift", ["--list".to_string()])?;
    let mut versions = vec![];
    for line in lines {
        if line.contains(DESCRIPTION_PREFIX) {
            let version = line
                .split_whitespace()
                .nth(2)
                .with_context(|| format!("Can't extract version from line {:?}", line))?;

            versions.push(version.to_string());
        }
    }
    versions.sort_unstable();
    versions.reverse();

    eprintln!("{}", "Existing snapshots (newest to oldest):".green());
    for (n, version) in versions.iter().enumerate() {
        let n = n + 1;
        let marker = if n > KEEP_ONLY_SNAPSHOTS {
            "<- to be removed".red().to_string()
        } else {
            String::new()
        };
        eprintln!("    {} {}", format!("{:>3}. {version}", n).yellow(), marker);
    }
    eprintln!(
        "{}",
        format!("Keeping only {} latest snapshots", KEEP_ONLY_SNAPSHOTS).green()
    );

    if let Some(snapshots_to_remove) = versions.get(KEEP_ONLY_SNAPSHOTS..) {
        for snapshot in snapshots_to_remove {
            spawn_and_forward_stdout_and_stderr(
                "timeshift",
                [
                    "--delete".to_string(),
                    "--snapshot".to_string(),
                    snapshot.to_string(),
                ],
            )?;
        }
    }

    eprintln!("{}", "Updating grub".green());

    spawn_and_forward_stdout_and_stderr("/usr/sbin/update-grub", [])?;

    eprintln!("{}", "Done".green());

    Ok(())
}
