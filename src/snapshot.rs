use crate::exec3;
use anyhow::Result;

pub struct Snapshot {
    pub timestamp: String,
    pub path: String,
}

impl Snapshot {
    pub fn all() -> Result<Vec<Snapshot>> {
        let lines = exec3("btrfs", "subvolume", "list", "/")?;
        let mut out = vec![];
        for line in lines {
            if let Some((_, timestamp)) = line.split_once(".snapshots/") {
                out.push(Snapshot {
                    timestamp: timestamp.to_string(),
                    path: format!(".snapshots/{timestamp}"),
                })
            }
        }
        Ok(out)
    }
}
