use std::{
    env,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};

fn reconnection_required(output: &str) -> bool {
    let lower = output.to_lowercase();
    return lower.contains("lost connection to pod") || lower.contains("timeout");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    loop {
        let (tx, rx) = mpsc::channel::<String>();

        let mut child = Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute command");

        let mut stderr = child.stderr.take();
        let mut stdout = child.stdout.take();

        thread::spawn(move || {
            for line in BufReader::new(stderr.as_mut().unwrap()).lines() {
                let value = line.ok().unwrap();
                if reconnection_required(&value) {
                    tx.send(String::from(&value)).unwrap();
                } else {
                    println!("\x1b[93mFatal Error\x1b[0m - {}", &value);
                    println!("\x1b[93mConnection can't be retried\x1b[0m");
                }
            }
        });

        thread::spawn(move || {
            for line in BufReader::new(stdout.as_mut().unwrap()).lines() {
                println!("{}", line.ok().unwrap())
            }
        });

        rx.recv().unwrap();
        println!("broken pipe - establishing new connection in 500ms");
        thread::sleep(Duration::from_millis(500));
        child.kill().unwrap();
    }
}
