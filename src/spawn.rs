use miette::{bail, Context as _, IntoDiagnostic, Report, Result};
use owo_colors::OwoColorize;
use std::{
    io::{BufRead as _, BufReader},
    process::Stdio,
};

pub(crate) fn spawn_and_forward_stdout_and_stderr(
    exe: impl AsRef<str>,
    args: impl IntoIterator<Item = String>,
) -> Result<Vec<String>> {
    let exe = exe.as_ref();
    let args = args.into_iter().collect::<Vec<_>>();

    eprintln!(
        "{} {} {} ...",
        "===== Spawning".green(),
        exe.yellow(),
        args.iter()
            .map(|e| e.cyan().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );

    let mut command = std::process::Command::new(exe);

    command.args(&args);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .into_diagnostic()
        .with_context(|| format!("failed to spawn {} {:?}", exe, args))?;

    let child_stdout = child
        .stdout
        .take()
        .with_context(|| format!("failed to get child's stdout of {} {:?}", exe, args))?;

    let child_stderr = child
        .stderr
        .take()
        .with_context(|| format!("failed to get child's stderr of {} {:?}", exe, args))?;

    let stdout_thread = std::thread::spawn(move || {
        let stdout_lines = BufReader::new(child_stdout).lines();
        let mut recorded = vec![];

        for line in stdout_lines {
            let line = line
                .into_diagnostic()
                .context("failed to read a line from stdout")?;
            println!("{}", line);
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
            eprintln!("{}", line);
        }
        Result::<_, Report>::Ok(())
    });

    let status = child
        .wait()
        .into_diagnostic()
        .context("failed to wait on child")?;

    if !status.success() {
        bail!(
            "failed to execute {} {:?}\nstatus code: {:?}",
            exe,
            args,
            status.code()
        );
    }

    if let Err(err) = stderr_thread.join() {
        bail!("failed to join stderr thread: {:?}", err);
    }

    match stdout_thread.join() {
        Ok(stdout_lines) => stdout_lines,
        Err(err) => bail!("failed to join stdout thread: {:?}", err),
    }
}
