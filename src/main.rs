use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::{thread, time::Duration};

fn reconnection_required(output: &str) -> bool {
    let lower = output.to_lowercase();
    return lower.contains("lost connection to pod") || lower.contains("timeout");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    loop {
        let mut child = Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute command");

        for line in BufReader::new(child.stderr.as_mut().unwrap()).lines() {
            let value = line.ok().unwrap();
            if reconnection_required(&value) {
                println!("broken pipe - establishing new connection");
                break;
            } else {
                println!("\x1b[93mFatal Error\x1b[0m - {}", &value);
                println!("\x1b[93mConnection can't be retried\x1b[0m");
                return;
            }
        }

        thread::sleep(Duration::from_millis(500));
        child.kill().unwrap();
    }
}

// E0417 12:59:21.114449   42312 portforward.go:340] error creating error stream for port 9200 -> 9200: Timeout occurred
// E0417 12:59:21.114448   42312 portforward.go:340] error creating error stream for port 9200 -> 9200: Timeout occurred
// E0417 13:01:01.211521   42312 portforward.go:233] lost connection to pod
// "core-elasticsearch-es-default-0", "9200:9200",
//./target/debug/ppf core-elasticsearch-es-default-0 9200:9200
