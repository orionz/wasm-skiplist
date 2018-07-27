
use std::time::{SystemTime};
use rand;

pub fn now() -> f64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => (n.as_secs() as f64 + (n.subsec_millis() as f64 / 1000.0)),
        Err(_) => 0.0
    }
}

pub fn log(s: &str) {
  println!("{}",s);
}

pub fn random() -> f64 {
  rand::random()
}
