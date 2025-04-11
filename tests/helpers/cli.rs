use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn run_reorganization(
    source: &str,
    destination: &str,
    embeddings_model_name: &str,
    llm_model_name: &str,
    llm_address: &str,
    qdrant_address: &str,
) {
    let args = [
        "run",
        "--",
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
    run_command_realtime("cargo", &args).expect("Failed to run command");
}

fn run_command_realtime(program: &str, args: &[&str]) -> std::io::Result<()> {
    let mut child = Command::new(program)
        .args(args)
        .env("RUST_BACKTRACE", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()) // rust will write building logs here and it's expected
        .spawn()?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let stdout_thread = std::thread::spawn(move || {
        for line in stdout_reader.lines().map_while(Result::ok) {
            println!("[stdout] {}", line);
        }
    });

    let stderr_thread = std::thread::spawn(move || {
        for line in stderr_reader.lines().map_while(Result::ok) {
            eprintln!("[stderr] {}", line);
        }
    });

    let status = child.wait()?;
    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    if !status.success() {
        eprintln!("Command exited with status: {}", status);
    }

    Ok(())
}
