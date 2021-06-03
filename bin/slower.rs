use std::io;
use std::time::Duration;

use slower::{Slower, DEFAULT_RATE_LIMIT};

/// Get the output's ratelimit for stdout via clap
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
    let stdin = io::stdin();
    let stdout = io::stdout();
    for s in Slower::new(stdin.lock(), stdout.lock(), rate_limit()) {
        if s == 0 {
            break;
        }
    }
}
