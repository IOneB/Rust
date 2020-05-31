use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let message = "Hello fellow slat";
    let width = message.chars().count();

    let lock = stdout();
    let mut writer = BufWriter::new(lock.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
