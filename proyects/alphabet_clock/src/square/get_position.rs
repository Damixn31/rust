use std::usize;

use chrono::{Local, Timelike};

use crate::one::one_pm::{
    a_quarter_to_two_pm, five_to_two_pm, one_and_a_half_pm, one_and_a_quarter_pm, one_and_five_pm,
    one_and_ten_pm, one_and_twenty_five_pm, one_and_twenty_pm, one_less_twenty_five_pm,
    one_less_twenty_pm, one_pm, ten_to_two_pm,
};
use crate::twelve::{
    it_is_a_quarter_past_twelve::it_is_a_quarter_past_twelve,
    it_is_five_to_twelve_o_clock::it_is_five_to_twelve_o_clock,
    it_is_half_past_twelve::it_is_half_past_twelve,
    it_is_ten_to_twelve_o_clock::it_is_ten_to_twelve_o_clock, it_is_twelve_am::it_is_twelve_am,
    it_is_twelve_and_five_am::it_is_twelve_and_five_am,
    it_is_twelve_and_five_pm::it_is_twelve_and_five_pm,
    it_is_twelve_and_ten_am::it_is_twelve_and_ten_am,
    it_is_twelve_and_ten_pm::it_is_twelve_and_ten_pm,
    it_is_twelve_less_one_fourth::it_is_twelve_less_one_fourth,
    it_is_twelve_less_twenty_five::it_is_twelve_less_twenty_five, it_is_twelve_pm::it_is_twelve_pm,
    it_is_twenty_five_past_twelve::it_is_twenty_five_past_twelve,
    it_is_twenty_past_twelve::it_is_twenty_past_twelve,
    it_is_twenty_to_one_o_clock::it_is_twenty_to_one_o_clock,
};

use crate::one::one_am::{
    a_quarter_to_two_am, five_to_two_am, one_am, one_and_a_half_am, one_and_a_quarter,
    one_and_five_am, one_and_ten_am, one_and_twenty_am, one_and_twenty_five_am, one_less_twenty_am,
    one_less_twenty_five_am, ten_to_two_am,
};

pub fn get_position_highlighted(hour_current: &str) -> Vec<(usize, usize)> {
    let hour_dt = Local::now().time();

    let (hour, minute) = (hour_dt.hour(), hour_dt.minute());

    match (hour, minute) {
        //AM
        (00, 0..=4) => it_is_twelve_am(),
        (00, 5..=9) => it_is_twelve_and_five_am(),
        (00, 10..=14) => it_is_twelve_and_ten_am(),
        (00, 15..=19) => it_is_a_quarter_past_twelve(),
        (00, 20..=24) => it_is_twenty_past_twelve(),
        (00, 25..=29) => it_is_twenty_five_past_twelve(),
        (00, 30..=34) => it_is_half_past_twelve(),
        (00, 35..=39) => it_is_twelve_less_twenty_five(),
        (00, 40..=44) => it_is_twenty_to_one_o_clock(),
        (00, 45..=49) => it_is_twelve_less_one_fourth(),
        (00, 50..=54) => it_is_ten_to_twelve_o_clock(),
        (00, 55..=59) => it_is_five_to_twelve_o_clock(),

        (1, 0..=4) => one_am(),
        (1, 5..=9) => one_and_five_am(),
        (1, 10..=14) => one_and_ten_am(),
        (1, 15..=19) => one_and_a_quarter(),
        (1, 20..=24) => one_and_twenty_am(),
        (1, 25..=29) => one_and_twenty_five_am(),
        (1, 30..=34) => one_and_a_half_am(),
        (1, 35..=39) => one_less_twenty_five_am(),
        (1, 40..=44) => one_less_twenty_am(),
        (1, 45..=49) => a_quarter_to_two_am(),
        (1, 50..=54) => ten_to_two_am(),
        (1, 55..=59) => five_to_two_am(),

        //PM
        (12, 0..=4) => it_is_twelve_pm(),
        (12, 5..=9) => it_is_twelve_and_five_pm(),
        (12, 10..=14) => it_is_twelve_and_ten_pm(),

        // -------- ONE
        (13, 0..=4) => one_pm(),
        (13, 5..=9) => one_and_five_pm(),
        (13, 10..=14) => one_and_ten_pm(),
        (13, 15..=19) => one_and_a_quarter_pm(),
        (13, 20..=24) => one_and_twenty_pm(),
        (13, 25..=29) => one_and_twenty_five_pm(),
        (13, 30..=34) => one_and_a_half_pm(),
        (13, 35..=39) => one_less_twenty_five_pm(), // aca es dos menos 25
        (13, 40..=44) => one_less_twenty_pm(),      // aca es dos menos 20
        (13, 45..=49) => a_quarter_to_two_pm(),     // aca es dos menos 15
        (13, 50..=54) => ten_to_two_pm(),           // aca es dos menos 10
        (13, 55..=59) => five_to_two_pm(),          // aca es dos menos 5
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
