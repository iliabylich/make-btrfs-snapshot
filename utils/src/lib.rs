use miette::{Context as _, IntoDiagnostic, Report, Result, bail};
use std::{
    io::{BufRead as _, BufReader},
    process::Stdio,
};

pub const NC: anstyle::Reset = anstyle::Reset;
pub const GREEN: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green)));
pub const YELLOW: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow)));
pub const RED: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)));
pub const CYAN: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan)));

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

pub fn exec(exe: impl AsRef<str>, args: impl IntoIterator<Item = String>) -> Result<Vec<String>> {
    let exe = exe.as_ref();
    let args = args.into_iter().collect::<Vec<_>>();

    eprintln!(
        "{GREEN}===== Spawning{NC} {YELLOW}{exe}{NC} {CYAN}{}{NC} ...",
        args.join(" ")
    );

    let mut command = std::process::Command::new(exe);

    command.args(&args);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .into_diagnostic()
        .with_context(|| format!("failed to spawn {exe} {args:?}"))?;

    let child_stdout = child
        .stdout
        .take()
        .with_context(|| format!("failed to get child's stdout of {exe} {args:?}"))?;

    let child_stderr = child
        .stderr
        .take()
        .with_context(|| format!("failed to get child's stderr of {exe} {args:?}"))?;

    let stdout_thread = std::thread::spawn(move || {
        let stdout_lines = BufReader::new(child_stdout).lines();
        let mut recorded = vec![];

        for line in stdout_lines {
            let line = line
                .into_diagnostic()
                .context("failed to read a line from stdout")?;
            eprintln!("{line}");
            recorded.push(line);
        }
        Ok(recorded)
    });

    let stderr_thread = std::thread::spawn(move || {
        let stderr_lines = BufReader::new(child_stderr).lines();
        for line in stderr_lines {
            let line = line
                .into_diagnostic()
                .context("failed to read a line from stderr")?;
            eprintln!("{line}");
        }
        Result::<_, Report>::Ok(())
    });

    let status = child
        .wait()
        .into_diagnostic()
        .context("failed to wait on child")?;

    if !status.success() {
        bail!(
            "failed to execute {exe} {args:?}\nstatus code: {:?}",
            status.code()
        );
    }

    if let Err(err) = stderr_thread.join() {
        bail!("failed to join stderr thread: {err:?}");
    }

    match stdout_thread.join() {
        Ok(stdout_lines) => stdout_lines,
        Err(err) => bail!("failed to join stdout thread: {err:?}"),
    }
}

pub fn exec0(exe: impl AsRef<str>) -> Result<Vec<String>> {
    exec(exe, [])
}

pub fn exec1(exe: impl AsRef<str>, arg: impl Into<String>) -> Result<Vec<String>> {
    exec(exe, [arg.into()])
}

pub fn exec2(
    exe: impl AsRef<str>,
    arg1: impl Into<String>,
    arg2: impl Into<String>,
) -> Result<Vec<String>> {
    exec(exe, [arg1.into(), arg2.into()])
}

pub fn exec3(
    exe: impl AsRef<str>,
    arg1: impl Into<String>,
    arg2: impl Into<String>,
    arg3: impl Into<String>,
) -> Result<Vec<String>> {
    exec(exe, [arg1.into(), arg2.into(), arg3.into()])
}

pub fn exec4(
    exe: impl AsRef<str>,
    arg1: impl Into<String>,
    arg2: impl Into<String>,
    arg3: impl Into<String>,
    arg4: impl Into<String>,
) -> Result<Vec<String>> {
    exec(exe, [arg1.into(), arg2.into(), arg3.into(), arg4.into()])
}

pub fn exec5(
    exe: impl AsRef<str>,
    arg1: impl Into<String>,
    arg2: impl Into<String>,
    arg3: impl Into<String>,
    arg4: impl Into<String>,
    arg5: impl Into<String>,
) -> Result<Vec<String>> {
    exec(
        exe,
        [
            arg1.into(),
            arg2.into(),
            arg3.into(),
            arg4.into(),
            arg5.into(),
        ],
    )
}
