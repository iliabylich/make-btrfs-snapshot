use miette::{Result, bail};
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

        let device = exec3("grub-probe", "--target", "device", "/")?;
        if device.len() != 1 {
            bail!("unexpected output of grub-probe: {device:?}")
        }
        let device = device.into_iter().next().unwrap();

        let uuid = exec4("grub-probe", "--device", &device, "--target", "fs_uuid")?;
        if uuid.len() != 1 {
            bail!("unexpected output of grub-probe: {uuid:?}")
        }
        let uuid = uuid.into_iter().next().unwrap();

        Ok(Self { path, device, uuid })
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.path, self.device, self.uuid)
    }
}
