use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;

fn main() {
    // Compile code.
    let mut child = Command::new("sudo")
        .args([
            "waydroid",
            "logcat"
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let stdout = child.stdout.take().unwrap();

    // Stream output.
    let lines = BufReader::new(stdout).lines();
    for line in lines {
        println!("{}", line.unwrap());
    }
    println!("Hello, world!");
}

// Example how to read from stdout of child process:
// https://stackoverflow.com/questions/72750736/run-command-stream-stdout-stderr-and-capture-results