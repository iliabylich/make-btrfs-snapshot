use anyhow::{Context, Result};

pub(crate) struct Fstab;

impl Fstab {
    pub(crate) fn mount_options_for_uuid(uuid: &str) -> Result<String> {
        let fstab = std::fs::read_to_string("/etc/fstab").context("failed to read /etc/fstab")?;

        let line = fstab
            .lines()
            .find(|line| line.contains(uuid))
            .with_context(|| format!("failed to find device {uuid} in /etc/fstab"))?;

        let options = line
            .split_whitespace()
            .find(|part| part.contains("subvol"))
            .with_context(|| {
                format!("device {uuid} is registered in /etc/fstab, but it has no mount options")
            })?;

        let clean = options
            .split(',')
            .filter(|opt| !opt.contains("subvol"))
            .collect::<Vec<_>>()
            .join(",");

        Ok(clean)
    }
}
