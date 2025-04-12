use miette::Result;
use utils::{GREEN, NC, RED, Snapshot, YELLOW, exec0, exec3, exec4};

const KEEP_ONLY_SNAPSHOTS: usize = 5;

fn main() -> Result<()> {
    eprintln!("{GREEN}Starting make-btrfs-snapshot{NC}");

    let now = chrono::Local::now()
        .format("%Y-%m-%d--%H-%M-%S")
        .to_string();

    exec4(
        "btrfs",
        "subvolume",
        "snapshot",
        "/",
        format!("/.snapshots/{now}"),
    )?;

    let mut paths = Snapshot::all()?
        .into_iter()
        .map(|s| s.path)
        .collect::<Vec<_>>();

    paths.sort_unstable();
    paths.reverse();

    eprintln!("{GREEN}Existing snapshots (newest to oldest):{NC}");
    for (n, path) in paths.iter().enumerate() {
        let n = n + 1;
        let marker = if n > KEEP_ONLY_SNAPSHOTS {
            format!("{RED}<- to be removed{NC}")
        } else {
            String::new()
        };
        eprintln!("    {YELLOW}{n:>3}. {path}{NC} {marker}");
    }
    eprintln!("{GREEN}Keeping only {KEEP_ONLY_SNAPSHOTS} latest snapshots{NC}");

    if let Some(paths_to_remove) = paths.get(KEEP_ONLY_SNAPSHOTS..) {
        for path in paths_to_remove {
            exec3("btrfs", "subvolume", "delete", format!("/{path}"))?;
        }
    }

    eprintln!("{GREEN}Updating grub{NC}");

    exec0("/usr/sbin/update-grub")?;

    eprintln!("{GREEN}Done{NC}");

    Ok(())
}
