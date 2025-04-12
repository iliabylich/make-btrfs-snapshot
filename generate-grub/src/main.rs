mod boot;
mod device;
mod fstab;
mod kernel;
mod template;

use device::Device;
use fstab::Fstab;
use kernel::Kernel;
use miette::Result;
use template::Template;
use utils::{CYAN, GREEN, NC, RED, Snapshot, YELLOW};

fn main() -> Result<()> {
    let kernels = Kernel::list()?;

    eprintln!("{YELLOW}Detected kernels:{NC}");
    if kernels.is_empty() {
        eprintln!("{RED}None, aborting.{NC}")
    } else {
        for kernel in kernels.iter() {
            eprintln!("   {CYAN}+{NC} {GREEN}{kernel}{NC}")
        }
    }

    let root = Device::new("/")?;
    let root_mount_options = Fstab::options_for_uuid(&root.uuid)?;
    eprintln!("{YELLOW}Root: {root}{NC}");
    eprintln!("{YELLOW}Mount options: {root_mount_options}{NC}");

    let boot = Device::new("/boot")?;
    eprintln!("{YELLOW}Boot: {boot}{NC}");

    let snapshots = Snapshot::all()?;
    eprintln!("{GREEN}Existing snapshots (newest to oldest):{NC}");
    for snapshot in snapshots.iter() {
        eprintln!("    {CYAN}+{NC} {GREEN}{}{NC}", snapshot.path);
    }

    let template = Template::new()?;

    for snapshot in snapshots.iter() {
        for kernel in kernels.iter() {
            let grub = template.render(&root, &root_mount_options, &boot, kernel, snapshot)?;
            println!("{grub}");
        }
    }

    Ok(())
}
