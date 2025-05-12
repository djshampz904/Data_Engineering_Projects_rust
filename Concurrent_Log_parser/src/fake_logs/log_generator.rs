use fake::{Fake, faker};
use chrono::{Utc, DateTime};
use std::fs::File;
use std::io::Write;


fn main() -> std::io::Result<()> {
    let mut file = File::create("generated_logs.log");

    for _ in 0..1000000 {
        let timestamp: DateTime<Utc> = Utc::now();
        let level = ["INFO", "WARM", "ERROR", "DEBUG"].choose().unwrap();
        let ip: String = faker::internet::en::IP().fake();
        let user_agent: String = faker::internet
    }
}