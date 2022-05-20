use std::{
    env,
    io::{BufRead, BufReader},
    process::{self, Command, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        println!(
            "\x1b[93m{}\x1b[0m",
            "yo, what am i supposed to be persisting??????"
        );
        process::exit(1)
    }

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
            let err_stream = stderr.as_mut().unwrap();
            for line in BufReader::new(err_stream).lines() {
                let value = line.ok().unwrap();
                tx.send(String::from(&value)).unwrap();
            }
        });

        thread::spawn(move || {
            for line in BufReader::new(stdout.as_mut().unwrap()).lines() {
                println!("{}", line.ok().unwrap())
            }
        });

        if let Some(value) = rx.recv().ok() {
            println!("\x1b[93m{}\x1b[0m", value);
        }

        println!("previous pipe broken, creating new one in 500ms");
        thread::sleep(Duration::from_millis(500));
        child.kill().unwrap();
    }
}
