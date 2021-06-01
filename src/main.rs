#[global_allocator]
static A: std::alloc::System = std::alloc::System;

use std::{
    io::{self, BufRead},
    mem,
    sync::mpsc::{self, Sender},
    thread::{sleep, spawn, JoinHandle},
    time::{Duration, Instant},
};

const DEFAULT_RATE_LIMIT: u64 = 200;

fn reader(sender: Sender<String>) -> JoinHandle<()> {
    spawn(move || {
        let input = io::stdin();
        let mut locked = input.lock();
        let mut buffer = String::new();
        while let Ok(size) = locked.read_line(&mut buffer) {
            if size == 0 {
                return;
            }
            sender.send(mem::take(&mut buffer)).unwrap();
        }
    })
}

fn rate_limit() -> Duration {
    let matches = clap::App::new("slower")
        .version("0.1")
        .author("Fuzen-py <me@fuzen.cafe>")
        .about("Slows down stdout output")
        .arg(
            clap::Arg::with_name("rate-limit")
                .short("t")
                .long("rate-limit")
                .help("output rate limit in miliseconds")
                .default_value("150")
                .takes_value(true),
        )
        .get_matches();
    let rl = matches
        .value_of("rate-limit")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_RATE_LIMIT);
    Duration::from_millis(rl)
}

fn main() {
    let rl = rate_limit();
    let (snd, rcv) = mpsc::channel::<String>();
    let _handle = reader(snd);
    let mut now = Instant::now();
    while let Ok(s) = rcv.recv() {
        if let Some(t) = rl.checked_sub(now.elapsed()) {
            sleep(t);
        }
        print!("{}", s);
        now = Instant::now();
    }
}
