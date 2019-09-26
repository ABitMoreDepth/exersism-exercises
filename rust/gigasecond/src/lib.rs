use chrono::{DateTime, Utc, Duration};
use core::ops::Add;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigasecond = Duration::seconds(1000000000);

    start.add(gigasecond)
}
