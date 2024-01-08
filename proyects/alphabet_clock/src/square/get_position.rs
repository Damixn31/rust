use std::usize;

use chrono::{Local, Timelike};

//use crate::hours;
use crate::twelve::{
    it_is_a_quarter_past_twelve::it_is_a_quarter_past_twelve,
    it_is_five_to_twelve_o_clock::it_is_five_to_twelve_o_clock,
    it_is_half_past_twelve::it_is_half_past_twelve,
    it_is_ten_to_twelve_o_clock::it_is_ten_to_twelve_o_clock, it_is_twelve::it_is_twelve,
    it_is_twelve_and_five::it_is_twelve_and_five, it_is_twelve_and_ten::it_is_twelve_and_ten,
    it_is_twelve_less_one_fourth::it_is_twelve_less_one_fourth,
    it_is_twelve_less_twenty_five::it_is_twelve_less_twenty_five,
    it_is_twenty_five_past_twelve::it_is_twenty_five_past_twelve,
    it_is_twenty_past_twelve::it_is_twenty_past_twelve,
    it_is_twenty_to_one_o_clock::it_is_twenty_to_one_o_clock,
};

pub fn get_position_highlighted(hour_current: &str) -> Vec<(usize, usize)> {
    let hour_dt = Local::now();

    let (hour, minute) = (hour_dt.hour(), hour_dt.minute());

    match (hour, minute) {
        (0, 0) => it_is_twelve(),
        (0, 5) => it_is_twelve_and_five(),
        (0, 10) => it_is_twelve_and_ten(),
        (0, 15) => it_is_a_quarter_past_twelve(),
        (0, 20) => it_is_twenty_past_twelve(),
        (0, 25) => it_is_twenty_five_past_twelve(),
        (0, 30) => it_is_half_past_twelve(),
        (0, 35) => it_is_twelve_less_twenty_five(),
        (0, 40) => it_is_twenty_to_one_o_clock(),
        (0, 45) => it_is_twelve_less_one_fourth(),
        (0, 50) => it_is_ten_to_twelve_o_clock(),
        (11, 43) => it_is_five_to_twelve_o_clock(),
        _ => Vec::new(),
    }

    //let hour = hour_dt.hour();
    //let minute = hour_dt.minute();

    //if hour == 00 && minute == 00 {
    //    it_is_twelve()
    //} else if hour == 00 && minute == 5 {
    //    it_is_twelve_and_five()
    //} else if hour == 00 && minute == 10 {
    //    it_is_twelve_and_ten()
    //} else if hour == 00 && minute == 15 {
    //    it_is_a_quarter_past_twelve()
    //} else if hour == 00 && minute == 20 {
    //    it_is_twenty_past_twelve()
    //} else {
    //    Vec::new()
    //}
}
