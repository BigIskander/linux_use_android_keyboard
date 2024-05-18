use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;
use regex::Regex;

fn myregex(text: &str) {
    let re = Regex::new(r"sendtextadb:(.{0,}$)").unwrap();
    let Some(caps) = re.captures(&text) else {
        return;
    };
    let result = &caps[1][1..];
    if result == "Ctrl+Enter" {
        println!("get Ctrl+Enter");
        Command::new("ctrlenter").args([text]).spawn().unwrap();
    } else {
        let re1 = Regex::new(r"text:(.{0,}$)").unwrap();
        let Some(caps1) = re1.captures(&result) else {
            return;
        };
        let result1 = &caps1[1];
        println!("get text:");
        println!("{}", &result1);
        let mut text = result1.escape_unicode().to_string();
        text = str::replace(&text, "}", "");
        text = str::replace(&text, "{", "");
        Command::new("typemetext").args([text]).spawn().unwrap();
    }
}

fn main() {
    let mut child = Command::new("getlogcat")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let stdout = child.stdout.take().unwrap();
    // Stream output.
    let lines = BufReader::new(stdout).lines();
    for line in lines {
        let text = line.unwrap();
        myregex(&text);
    }
}

// Example how to read from stdout of child process:
// https://stackoverflow.com/questions/72750736/run-command-stream-stdout-stderr-and-capture-results

// Regex doc:
// https://docs.rs/regex/latest/regex/