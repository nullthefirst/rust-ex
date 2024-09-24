use time::{PrimitiveDateTime as DateTime, Duration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let duration = Duration::seconds(1000000000);

    let giga_later = start + duration;
    return  giga_later;

    // todo!("What time is a gigasecond later than {start}");
}
