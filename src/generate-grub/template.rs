use crate::{Device, Kernel};
use anyhow::{Result, ensure};
use make_btrfs_snapshot::Snapshot;

const TEMPLATE: &str = include_str!("template");

pub(crate) struct Template;

impl Template {
    pub(crate) fn render(
        root: &Device,
        root_mount_options: &str,
        boot: &Device,
        kernel: &Kernel,
        snapshot: &Snapshot,
    ) -> Result<String> {
        let template = TEMPLATE
            .replace("{{boot_uuid}}", &boot.uuid)
            .replace("{{initramfs_path}}", &kernel.initramfs)
            .replace("{{kernel_path}}", &kernel.filepath)
            .replace("{{kernel_version_arch}}", &kernel.version_arch)
            .replace("{{mount_options}}", &root_mount_options)
            .replace("{{root_uuid}}", &root.uuid)
            .replace("{{snapshot_path}}", &snapshot.path)
            .replace("{{snapshot_timestamp}}", &snapshot.timestamp);

        ensure!(!template.contains("{{"));
        ensure!(!template.contains("}}"));

        Ok(template)
    }
}
