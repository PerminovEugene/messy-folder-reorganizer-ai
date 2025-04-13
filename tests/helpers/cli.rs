use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

pub fn run_reorganization(
    source: &str,
    destination: &str,
    embeddings_model_name: &str,
    llm_model_name: &str,
    llm_address: &str,
    qdrant_address: &str,
    mode: OutputMode,
) -> std::io::Result<()> {
    let args = [
        "process",
        "--source",
        source,
        "--destination",
        destination,
        "-L",
        llm_model_name,
        "-E",
        embeddings_model_name,
        "-n",
        llm_address,
        "-q",
        qdrant_address,
        "-F",
        "-R",
    ];
    println!("--->");

    let binary_path = "./target/debug/messy-folder-reorganizer-ai";
    run_command_realtime(binary_path, &args, mode)
}

pub enum OutputMode {
    ToConsole,
    ToFile(String),
    Silent,
}

fn setup_command_and_logging(
    program: &str,
    args: &[&str],
    output: &OutputMode,
) -> std::io::Result<(Command, Option<File>)> {
    let mut command = Command::new(program);
    command.args(args);
    command.env("RUST_BACKTRACE", "1");

    let (stdout, stderr, log_file): (Stdio, Stdio, Option<File>) = match output {
        OutputMode::ToConsole => (Stdio::piped(), Stdio::piped(), None),
        OutputMode::ToFile(path) => {
            println!("path {:?}", path);

            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?;
            let file_for_stderr = file.try_clone()?; // for separate use in stderr thread
            println!("path {:?}", path);

            (Stdio::piped(), Stdio::from(file_for_stderr), Some(file))
        }
        OutputMode::Silent => (Stdio::null(), Stdio::null(), None),
    };

    println!("command {:?}", command);

    command.stdout(stdout).stderr(stderr);
    Ok((command, log_file))
}

fn spawn_output_thread<R: std::io::Read + Send + 'static>(
    stream: R,
    label: &'static str,
    mut output: OutputTarget,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines().map_while(Result::ok) {
            let msg = format!("[{}] {}\n", label, line);
            output.write(&msg);
        }
    })
}

enum OutputTarget {
    ConsoleStdout,
    ConsoleStderr,
    File(File),
}

impl OutputTarget {
    fn write(&mut self, line: &str) {
        match self {
            OutputTarget::ConsoleStdout => print!("{}", line),
            OutputTarget::ConsoleStderr => eprint!("{}", line),
            OutputTarget::File(file) => {
                let _ = file.write_all(line.as_bytes());
            }
        }
    }
}

pub fn run_command_realtime(
    program: &str,
    args: &[&str],
    output: OutputMode,
) -> std::io::Result<()> {
    println!("args: {}", args.join(" "));

    let (mut command, log_file) = setup_command_and_logging(program, args, &output)?;
    println!("a>> ");

    assert!(
        std::path::Path::new("./target/debug/messy-folder-reorganizer-ai").exists(),
        "Binary not built. Run `cargo build` first."
    );
    let mut child = command.spawn()?;

    let mut stdout_thread = None;
    let mut stderr_thread = None;

    if let Some(stdout) = child.stdout.take() {
        let log_for_stdout = match &output {
            OutputMode::ToFile(_) => Some(log_file.as_ref().unwrap().try_clone()?),
            OutputMode::ToConsole => None,
            OutputMode::Silent => None,
        };

        let target = match log_for_stdout {
            Some(file) => OutputTarget::File(file),
            None => OutputTarget::ConsoleStdout,
        };

        stdout_thread = Some(spawn_output_thread(stdout, "stdout", target));
    }

    if let Some(stderr) = child.stderr.take() {
        let target = match &output {
            OutputMode::ToFile(_) => OutputTarget::File(log_file.unwrap()),
            OutputMode::ToConsole => OutputTarget::ConsoleStderr,
            OutputMode::Silent => OutputTarget::ConsoleStderr, // you can ignore this too
        };

        stderr_thread = Some(spawn_output_thread(stderr, "stderr", target));
    }

    let status = child.wait()?;

    if let Some(handle) = stdout_thread {
        handle.join().unwrap();
    }

    if let Some(handle) = stderr_thread {
        handle.join().unwrap();
    }

    if !status.success() {
        eprintln!("Command exited with status: {}", status);
    }

    Ok(())
}
