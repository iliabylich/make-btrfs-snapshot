use miette::{Context as _, IntoDiagnostic as _, Result, bail};

pub(crate) struct Fstab;

impl Fstab {
    pub(crate) fn options_for_uuid(uuid: &str) -> Result<String> {
        let fstab = std::fs::read_to_string("/etc/fstab")
            .into_diagnostic()
            .context("failed to read /etc/fstab")?;

        for line in fstab.lines() {
            if line.contains(uuid) {
                for part in line.split_whitespace() {
                    if part.contains("subvol") {
                        let options = part
                            .split(',')
                            .filter(|opt| !opt.contains("subvol"))
                            .collect::<Vec<_>>()
                            .join(",");
                        return Ok(options);
                    }
                }
                bail!("device {uuid} is registered in /etc/fstab, but it has no mount options")
            }
        }

        bail!("failed to find device {uuid} in /etc/fstab")
    }
}
