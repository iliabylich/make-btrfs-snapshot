use miette::{Context as _, IntoDiagnostic as _, Result};
use utils::Snapshot;

use crate::{Device, Kernel};

pub(crate) struct Template {
    engine: upon::Engine<'static>,
}

impl Template {
    pub(crate) fn new() -> Result<Self> {
        let mut engine = upon::Engine::new();

        engine
            .add_template("grub", include_str!("./template"))
            .into_diagnostic()
            .context("failed to add template")?;

        Ok(Self { engine })
    }

    pub(crate) fn render(
        &self,
        root: &Device,
        root_mount_options: &str,
        boot: &Device,
        kernel: &Kernel,
        snapshot: &Snapshot,
    ) -> Result<String> {
        self.engine
            .template("grub")
            .render(upon::value! {
                boot_uuid: &boot.uuid,
                initramfs_path: &kernel.initramfs,
                kernel_path: &kernel.filepath,
                kernel_version_arch: &kernel.version_arch,
                mount_options: &root_mount_options,
                root_uuid: &root.uuid,
                snapshot_path: &snapshot.path,
                snapshot_timestamp: &snapshot.timestamp,
            })
            .to_string()
            .into_diagnostic()
            .context("failed to render grub template")
    }
}
