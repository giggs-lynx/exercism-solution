extern crate chrono;
use chrono::{DateTime, Utc, Duration};


// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let offset: i64 = 1_000_000_000;
    start + Duration::seconds(offset)
}