use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn generate_short_code() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;

    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    let value = timestamp ^ counter;

    to_base62(value)
}

pub fn to_base62(mut value: u64) -> String {
    println!("Generating short code for value: {}", value);
    const CHARSET: &[u8] =
        b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if value == 0 {
        return "0".to_string();
    }

    let mut result = Vec::new();

    while value > 0 {
        result.push(CHARSET[(value % 62) as usize] as char);
        value /= 62;
    }

    result.iter().rev().collect()
}