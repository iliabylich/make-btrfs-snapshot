use crate::boot;
use anyhow::Result;
use utils::{NC, YELLOW};

#[derive(Debug)]
pub(crate) struct Kernel {
    pub(crate) filepath: String,
    pub(crate) initramfs: String,
    pub(crate) version_arch: String,
}

impl Kernel {
    pub(crate) fn list() -> Result<Vec<Self>> {
        let filelist = boot::ls()?;
        let kernels = filelist
            .into_iter()
            .flat_map(|filename| Self::new(filename))
            .collect();
        Ok(kernels)
    }

    fn new(filename: String) -> Option<Self> {
        let version_arch = None
            .or_else(|| filename.strip_prefix("vmlinuz-"))
            .or_else(|| filename.strip_prefix("vmlinux-"))
            .or_else(|| filename.strip_prefix("kernel-"))?
            .to_string();

        if let Some(initramfs) = find_initramfs(&version_arch) {
            Some(Self {
                filepath: format!("/boot/{filename}"),
                initramfs,
                version_arch: version_arch.to_string(),
            })
        } else {
            eprintln!("{YELLOW}Failed to find initramfs for {filename}{NC}");
            None
        }
    }
}

fn find_initramfs(version_arch: &str) -> Option<String> {
    const PREFIXES: &[&str] = &["initrd.img", "initramfs", "initrd"];
    for prefix in PREFIXES {
        let filepath = format!("/boot/{prefix}-{version_arch}");
        if std::fs::exists(&filepath).is_ok_and(|v| v) {
            return Some(filepath);
        }
    }
    None
}

impl std::fmt::Display for Kernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version_arch)
    }
}
