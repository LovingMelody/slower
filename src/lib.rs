#[global_allocator]
static A: std::alloc::System = std::alloc::System;
use std::{
    fmt::Debug,
    io::{self, BufRead, BufReader, Read, Write},
    thread::sleep,
    time::{Duration, Instant},
};

// TODO: change default for old dos delays
/// Default ratelimit
pub const DEFAULT_RATE_LIMIT: u64 = 200;

/// Rate limit stdout
pub struct Slower<R: Read, W: Write> {
    writer: W,
    reader: BufReader<R>,
    //stream: VecDeque<u8>,
    rate_limit: Duration,
    last_run: Instant,
    eof: bool,
}

impl<R: Read, W: Write> Debug for Slower<R, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(last_run: {:?}, rate_limit: {:?}, eof: {:?} )",
            self.last_run, self.rate_limit, self.eof
        )
    }
}

impl<R: Read, W: Write> Slower<R, W> {
    pub fn new(reader: R, writer: W, rate_limit: Duration) -> Self {
        Self {
            reader: BufReader::new(reader),
            writer,
            rate_limit,
            last_run: Instant::now(),
            eof: false,
        }
    }
    pub fn write(&mut self) -> Result<usize, std::io::Error> {
        if self.eof {
            return Err(std::io::Error::new(
                io::ErrorKind::ConnectionReset,
                "EOF bit recieved",
            ));
        }
        let mut buffer = String::new();
        let len = self.reader.read_line(&mut buffer)?;
        if len == 0 {
            self.eof = true;
        }
        // TODO: First output shouldnt be restricted
        if let Some(t) = self.rate_limit.checked_sub(self.last_run.elapsed()) {
            sleep(t);
        }
        self.writer.write_all(buffer.as_bytes())?;
        self.last_run = Instant::now();
        Ok(len)
    }
}

impl<R: Read, W: Write> std::iter::Iterator for Slower<R, W> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.write().ok()
    }
}

/// Default stdout, stdin
/// WARNING: Does not lock either stdin / stdout
/// Please use `Slower::new(stdin.lock(), stdout.lock(), rate_limit())` instead
/// EX:
/// ```ignore
///    let stdin = std::io::stdin();
///    let stdout = std::io::stdout();
///     Slower::new(stdin.lock(), stdout.lock(), rate_limit()
/// ```
impl<'i> Default for Slower<io::Stdin, io::Stdout> {
    fn default() -> Self {
        let stdout = io::stdout();
        let stdin = io::stdin();
        Self {
            writer: stdout,
            reader: BufReader::new(stdin),
            rate_limit: Duration::from_millis(DEFAULT_RATE_LIMIT),
            eof: false,
            last_run: Instant::now(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::{io::BufRead, time::Instant};

    #[test]
    fn correctness() {
        // Make sure what goes out is exactly what goes in
        let rate_limit = std::time::Duration::from_secs(0);
        let reader = "Hello World\nThis is a test\nDoes it work?".as_bytes();
        let mut buffer: Vec<u8> = Vec::with_capacity(40);
        let writer = std::io::Cursor::new(&mut buffer);
        let slower = crate::Slower::new(reader, writer, rate_limit);
        for _ in slower {}
        assert_eq!(reader, buffer);
    }

    #[test]
    fn time() {
        let rate_limit = std::time::Duration::from_secs(1);
        let reader = "This Should Take\n3 Seconds".as_bytes();
        let mut buffer: Vec<u8> = Vec::with_capacity(40);
        let writer = std::io::Cursor::new(&mut buffer);
        let expected_seconds = (reader.lines().count() + 1) as u64;
        let slower = crate::Slower::new(reader, writer, rate_limit);
        let t = Instant::now();
        for _ in slower {}
        assert_eq!(reader, buffer);
        assert_eq!(t.elapsed().as_secs(), expected_seconds);
    }
}
