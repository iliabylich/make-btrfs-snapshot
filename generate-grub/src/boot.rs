use miette::{Context as _, IntoDiagnostic as _, Result};

pub(crate) fn ls() -> Result<Vec<String>> {
    let mut out = vec![];
    for entry in std::fs::read_dir("/boot")
        .into_diagnostic()
        .context("failed to read /boot")?
    {
        let entry = entry
            .into_diagnostic()
            .context("failed to read /boot entry")?;
        let filename = entry.file_name();
        let Some(filename) = filename.to_str() else {
            eprintln!("failed to convert filename {filename:?} to utf-8 string");
            continue;
        };
        out.push(filename.to_string())
    }
    Ok(out)
}
