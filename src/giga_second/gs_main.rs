


use crate::type_input;
// Your task is to determine the date and time one gigasecond after a certain date.A gigasecond is one thousand million seconds. That is a one with nine zeros after it.If you were born on January 24th, 2015 at 22:00 (10:00:00pm), then you would be a gigasecond old on October 2nd, 2046 at 23:46:40 (11:46:40pm).

use chrono::{NaiveDateTime, Duration};
use chrono::format::ParseError;

use super::constants;

fn parse_datetime(input: &str) -> Result<NaiveDateTime, ParseError> {
    let format = "%Y-%m-%d %H:%M:%S";
    NaiveDateTime::parse_from_str(input, format)
}

pub fn giga_second() {
    let datetime_str = type_input::ip_main::string_input();
    match parse_datetime(&datetime_str) {
        Ok(datetime) => {
            let giga_date = datetime + Duration::seconds(constants::GIGA_SECOND);
            println!("{}", giga_date);
        },
        Err(e) => println!("Failed to parse datetime: {}", e),
    }
}
