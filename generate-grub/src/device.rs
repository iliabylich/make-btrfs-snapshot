use anyhow::{Context, Result, ensure};
use utils::{exec3, exec4};

#[derive(Debug)]
pub(crate) struct Device {
    pub(crate) path: String,
    pub(crate) device: String,
    pub(crate) uuid: String,
}

impl Device {
    pub(crate) fn new(path: impl Into<String>) -> Result<Self> {
        let path = path.into();

        let device = sole(exec3("grub-probe", "--target", "device", "/")?)
            .context("expected exactly on item in output of grub-probe")?;

        let uuid = sole(exec4(
            "grub-probe",
            "--device",
            &device,
            "--target",
            "fs_uuid",
        )?)
        .context("expected exactly on item in output of grub-probe")?;

        Ok(Self { path, device, uuid })
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "path={} device={} uuid={}",
            self.path, self.device, self.uuid
        )
    }
}

fn sole<T>(v: Vec<T>) -> Result<T> {
    let mut iter = v.into_iter();
    let item = iter.next().context("empty list")?;
    ensure!(iter.next().is_none(), "list has more than 1 item");
    Ok(item)
}
