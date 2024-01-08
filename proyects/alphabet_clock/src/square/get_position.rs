use std::usize;

use chrono::{Local, Timelike};

//use crate::hours;
use crate::hours::it_is_a_quarter_past_twelve::it_is_a_quarter_past_twelve;
use crate::hours::it_is_twelve::it_is_twelve;
use crate::hours::it_is_twelve_and_five::it_is_twelve_and_five;
use crate::hours::it_is_twelve_and_ten::it_is_twelve_and_ten;
use crate::hours::it_is_twenty_past_twelve::it_is_twenty_past_twelve;

pub fn get_position_highlighted(hour_current: &str) -> Vec<(usize, usize)> {
    let hour_dt = Local::now();

    let hour = hour_dt.hour();
    let minute = hour_dt.minute();

    if hour == 00 && minute == 00 {
        it_is_twelve()
    } else if hour == 00 && minute == 5 {
        it_is_twelve_and_five()
    } else if hour == 00 && minute == 10 {
        it_is_twelve_and_ten()
    } else if hour == 00 && minute == 15 {
        it_is_a_quarter_past_twelve()
    } else if hour == 00 && minute == 20 {
        it_is_twenty_past_twelve()
    } else {
        Vec::new()
    }
}
