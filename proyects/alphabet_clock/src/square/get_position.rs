use std::usize;

use chrono::{Local, Timelike};

use crate::five::five_am::{
    five_am, five_and_a_quarter_am, five_and_five_am, five_and_half_am, five_and_ten_am,
    five_and_twenty_am, five_and_twenty_five_am, six_less_a_quarter_am, six_less_five_am,
    six_less_ten_am, six_less_twenty_am, six_less_twenty_five_am,
};
use crate::five::five_pm::{
    five_and_a_quarter_pm, five_and_five_pm, five_and_half_pm, five_and_ten_pm,
    five_and_twenty_five_pm, five_and_twenty_pm, five_pm, six_less_a_quarter_pm, six_less_five_pm,
    six_less_ten_pm, six_less_twenty_five_pm, six_less_twenty_pm,
};
use crate::four::four_am::{
    five_less_a_quarter_am, five_less_five_am, five_less_ten_am, five_less_twenty_am,
    five_less_twenty_five_am, four_am, four_and_a_quarter_am, four_and_five_am, four_and_half_am,
    four_and_ten_am, four_and_twenty_am, four_and_twenty_five_am,
};
use crate::four::four_pm::{
    five_less_a_quarter_pm, five_less_five_pm, five_less_ten_pm, five_less_twenty_five_pm,
    five_less_twenty_pm, four_and_a_quarter_pm, four_and_five_pm, four_and_half_pm,
    four_and_ten_pm, four_and_twenty_five_pm, four_and_twenty_pm, four_pm,
};
use crate::one::one_am::{
    a_quarter_to_two_am, five_to_two_am, one_am, one_and_a_half_am, one_and_a_quarter,
    one_and_five_am, one_and_ten_am, one_and_twenty_am, one_and_twenty_five_am, ten_to_two_am,
    two_less_twenty_am, two_less_twenty_five_am,
};
use crate::one::one_pm::{
    a_quarter_to_two_pm, five_to_two_pm, one_and_a_half_pm, one_and_a_quarter_pm, one_and_five_pm,
    one_and_ten_pm, one_and_twenty_five_pm, one_and_twenty_pm, one_pm, ten_to_two_pm,
    two_less_twenty_five_pm, two_less_twenty_pm,
};

use crate::six::six_am::{
    seven_less_a_quarter_am, seven_less_five_am, seven_less_ten_am, seven_less_twenty_am,
    seven_less_twenty_five_am, six_am, six_and_a_quarter_am, six_and_five_am, six_and_half_am,
    six_and_ten_am, six_and_twenty_am, six_and_twenty_five_am,
};
use crate::six::six_pm::{
    seven_less_a_quarter_pm, seven_less_five_pm, seven_less_ten_pm, seven_less_twenty_five_pm,
    seven_less_twenty_pm, six_and_a_quarter_pm, six_and_five_pm, six_and_half_pm, six_and_ten_pm,
    six_and_twenty_five_pm, six_and_twenty_pm, six_pm,
};
use crate::twelve::twelve_am::{
    a_quarter_to_one_am, it_is_half_past_twelve_am, one_less_five_am, one_less_ten_am,
    one_less_twenty_am, one_less_twenty_five_am, twelve_am, twelve_and_five_am,
    twelve_and_quarter_am, twelve_and_ten_am, twelve_and_twenty_am, twelve_and_twenty_five_am,
};
use crate::twelve::twelve_pm::{
    a_quarter_to_one_pm, it_is_half_past_twelve_pm, one_less_five_pm, one_less_ten_pm,
    one_less_twenty_five_pm, one_less_twenty_pm, twelve_and_five_pm, twelve_and_quarter_pm,
    twelve_and_ten_pm, twelve_and_twenty_five_pm, twelve_and_twenty_pm, twelve_pm,
};
use crate::two::two_am::{
    three_less_a_quarter_am, three_less_five_am, three_less_ten_am, three_less_twenty_am,
    three_less_twenty_five_am, two_am, two_and_a_quarter_am, two_and_five_am, two_and_half_am,
    two_and_ten_am, two_and_twenty_am, two_and_twenty_five_am,
};
use crate::two::two_pm::{
    three_less_a_quarter_pm, three_less_five_pm, three_less_ten_pm, three_less_twenty_five_pm,
    three_less_twenty_pm, two_and_a_quarter_pm, two_and_five_pm, two_and_half_pm, two_and_ten_pm,
    two_and_twenty_five_pm, two_and_twenty_pm, two_pm,
};

use crate::three::three_am::{
    four_less_a_quarter, four_less_five_am, four_less_ten_am, four_less_twenty_am,
    four_less_twenty_five_am, three_am, three_and_a_quarter_am, three_and_five_am,
    three_and_half_am, three_and_ten_am, three_and_twenty_am, three_and_twenty_five_am,
};

use crate::three::three_pm::{
    four_less_five_pm, four_less_ten_pm, four_less_twenty_five_pm, four_less_twenty_pm,
    three_and_a_quarter_pm, three_and_five_pm, three_and_half_pm, three_and_ten_pm,
    three_and_twenty_five_pm, three_and_twenty_pm, three_pm,
};

pub fn get_position_highlighted(hour_current: &str) -> Vec<(usize, usize)> {
    let hour_dt = Local::now().time();

    let (hour, minute) = (hour_dt.hour(), hour_dt.minute());

    match (hour, minute) {
        //AM
        // ---------------- 12
        (00, 0..=4) => twelve_am(),
        (00, 5..=9) => twelve_and_five_am(),
        (00, 10..=14) => twelve_and_ten_am(),
        (00, 15..=19) => twelve_and_quarter_am(),
        (00, 20..=24) => twelve_and_twenty_am(),
        (00, 25..=29) => twelve_and_twenty_five_am(),
        (00, 30..=34) => it_is_half_past_twelve_am(),
        (00, 35..=39) => one_less_twenty_five_am(),
        (00, 40..=44) => one_less_twenty_am(),
        (00, 45..=49) => a_quarter_to_one_am(),
        (00, 50..=54) => one_less_ten_am(),
        (00, 55..=59) => one_less_five_am(),

        // ----------------- 1
        (1, 0..=4) => one_am(),
        (1, 5..=9) => one_and_five_am(),
        (1, 10..=14) => one_and_ten_am(),
        (1, 15..=19) => one_and_a_quarter(),
        (1, 20..=24) => one_and_twenty_am(),
        (1, 25..=29) => one_and_twenty_five_am(),
        (1, 30..=34) => one_and_a_half_am(),
        (1, 35..=39) => two_less_twenty_five_am(),
        (1, 40..=44) => two_less_twenty_am(),
        (1, 45..=49) => a_quarter_to_two_am(),
        (1, 50..=54) => ten_to_two_am(),
        (1, 55..=59) => five_to_two_am(),

        // ---------------2
        (2, 0..=4) => two_am(),
        (2, 5..=9) => two_and_five_am(),
        (2, 10..=14) => two_and_ten_am(),
        (2, 15..=19) => two_and_a_quarter_am(),
        (2, 20..=24) => two_and_twenty_am(),
        (2, 25..=29) => two_and_twenty_five_am(),
        (2, 30..=34) => two_and_half_am(),
        (2, 35..=39) => three_less_twenty_five_am(),
        (2, 40..=44) => three_less_twenty_am(),
        (2, 45..=49) => three_less_a_quarter_am(),
        (2, 50..=54) => three_less_ten_am(),
        (2, 55..=59) => three_less_five_am(),

        // ---------- 3
        (3, 0..=4) => three_am(),
        (3, 5..=9) => three_and_five_am(),
        (3, 10..=14) => three_and_ten_am(),
        (3, 15..=19) => three_and_a_quarter_am(),
        (3, 20..=24) => three_and_twenty_am(),
        (3, 25..=29) => three_and_twenty_five_am(),
        (3, 30..=34) => three_and_half_am(),
        (3, 35..=39) => four_less_twenty_five_am(),
        (3, 40..=44) => four_less_twenty_am(),
        (3, 45..=49) => four_less_a_quarter(),
        (3, 50..=54) => four_less_ten_am(),
        (3, 55..=59) => four_less_five_am(),

        // ---------- 4
        (4, 0..=4) => four_am(),
        (4, 5..=9) => four_and_five_am(),
        (4, 10..=14) => four_and_ten_am(),
        (4, 15..=19) => four_and_a_quarter_am(),
        (4, 20..=24) => four_and_twenty_am(),
        (4, 25..=29) => four_and_twenty_five_am(),
        (4, 30..=34) => four_and_half_am(),
        (4, 35..=39) => five_less_twenty_five_am(),
        (4, 40..=44) => five_less_twenty_am(),
        (4, 45..=49) => five_less_a_quarter_am(),
        (4, 50..=54) => five_less_ten_am(),
        (4, 55..=59) => five_less_five_am(),

        // --------------5
        (5, 0..=4) => five_am(),
        (5, 5..=9) => five_and_five_am(),
        (5, 10..=14) => five_and_ten_am(),
        (5, 15..=19) => five_and_a_quarter_am(),
        (5, 20..=24) => five_and_twenty_am(),
        (5, 25..=29) => five_and_twenty_five_am(),
        (5, 30..=34) => five_and_half_am(),
        (5, 35..=39) => six_less_twenty_five_am(),
        (5, 40..=44) => six_less_twenty_am(),
        (5, 45..=49) => six_less_a_quarter_am(),
        (5, 50..=54) => six_less_ten_am(),
        (5, 55..=59) => six_less_five_am(),

        // ------  6
        (6, 0..=4) => six_am(),
        (6, 5..=9) => six_and_five_am(),
        (6, 10..=14) => six_and_ten_am(),
        (6, 15..=19) => six_and_a_quarter_am(),
        (6, 20..=24) => six_and_twenty_am(),
        (6, 25..=29) => six_and_twenty_five_am(),
        (6, 30..=34) => six_and_half_am(),
        (6, 35..=39) => seven_less_twenty_five_am(),
        (6, 40..=44) => seven_less_twenty_am(),
        (6, 45..=49) => seven_less_a_quarter_am(),
        (6, 50..=54) => seven_less_ten_am(),
        (6, 55..=59) => seven_less_five_am(),

        //PM
        //  ------------- TWELVE
        (12, 0..=4) => twelve_pm(),
        (12, 5..=9) => twelve_and_five_pm(),
        (12, 10..=14) => twelve_and_ten_pm(),
        (12, 15..=19) => twelve_and_quarter_pm(),
        (12, 20..=24) => twelve_and_twenty_pm(),
        (12, 25..=29) => twelve_and_twenty_five_pm(),
        (12, 30..=34) => it_is_half_past_twelve_pm(),
        (12, 35..=39) => one_less_twenty_five_pm(),
        (12, 40..=44) => one_less_twenty_pm(),
        (12, 45..=49) => a_quarter_to_one_pm(),
        (12, 50..=54) => one_less_ten_pm(),
        (12, 55..=59) => one_less_five_pm(),

        // -------- ONE
        (13, 0..=4) => one_pm(),
        (13, 5..=9) => one_and_five_pm(),
        (13, 10..=14) => one_and_ten_pm(),
        (13, 15..=19) => one_and_a_quarter_pm(),
        (13, 20..=24) => one_and_twenty_pm(),
        (13, 25..=29) => one_and_twenty_five_pm(),
        (13, 30..=34) => one_and_a_half_pm(),
        (13, 35..=39) => two_less_twenty_five_pm(), // cambiar
        (13, 40..=44) => two_less_twenty_pm(),
        (13, 45..=49) => a_quarter_to_two_pm(),
        (13, 50..=54) => ten_to_two_pm(),
        (13, 55..=59) => five_to_two_pm(), //cambiar

        // --------- TWO
        //(14, 0..=4) => two_pm(),
        //(14, 5..=9) => two_and_five_pm(),
        //(14, 10..=14) => two_and_ten_pm(),
        //(14, 15..=19) => two_and_a_quarter_pm(),
        //(14, 20..=24) => two_and_twenty_pm(),
        //(14, 25..=29) => two_and_twenty_five_pm(),
        //(14, 30..=34) => two_and_half_pm(),
        //(14, 35..=39) => three_less_twenty_five_pm(),
        //(14, 40..=44) => three_less_twenty_pm(),
        //(14, 45..=49) => three_less_a_quarter_pm(),
        //(14, 50..=54) => three_less_ten_pm(),
        //(14, 55..=59) => three_less_five_pm(),

        // ------- THREE
        //(15, 0..=4) => three_pm(),
        //(15, 5..=9) => three_and_five_pm(),
        //(15, 10..=14) => three_and_ten_pm(),
        //(15, 15..=19) => three_and_a_quarter_pm(),
        //(15, 20..=24) => three_and_twenty_pm(),
        //(15, 25..=29) => three_and_twenty_five_pm(),
        //(15, 30..=34) => three_and_half_pm(),
        //(15, 35..=39) => four_less_twenty_five_pm(),
        //(15, 40..=44) => four_less_twenty_pm(),
        //(15, 45..=49) => four_less_twenty_five_pm(),
        //(15, 50..=54) => four_less_ten_pm(),
        //(15, 55..=59) => four_less_five_pm(),

        // -------- FOUR
        (16, 0..=4) => four_pm(),
        (16, 5..=9) => four_and_five_pm(),
        (16, 10..=14) => four_and_ten_pm(),
        (16, 15..=19) => four_and_a_quarter_pm(),
        (16, 20..=24) => four_and_twenty_pm(),
        (16, 25..=29) => four_and_twenty_five_pm(),
        (16, 30..=34) => four_and_half_pm(),
        (16, 35..=39) => five_less_twenty_five_pm(),
        (16, 40..=44) => five_less_twenty_pm(),
        (16, 45..=49) => five_less_a_quarter_pm(),
        (16, 50..=54) => five_less_ten_pm(),
        (16, 55..=59) => five_less_five_pm(),

        //-------- FIVE
        (17, 0..=4) => five_pm(),
        (17, 5..=9) => five_and_five_pm(),
        (17, 10..=14) => five_and_ten_pm(),
        (17, 15..=19) => five_and_a_quarter_pm(),
        (17, 20..=24) => five_and_twenty_pm(),
        (17, 25..=29) => five_and_twenty_five_pm(),
        (17, 30..=34) => five_and_half_pm(),
        (17, 35..=39) => six_less_twenty_five_pm(),
        (17, 40..=44) => six_less_twenty_pm(),
        (17, 45..=49) => six_less_a_quarter_pm(),
        (17, 50..=54) => six_less_ten_pm(),
        (17, 55..=59) => six_less_five_pm(),

        // -------------- SIX
        (18, 0..=4) => six_pm(),
        (18, 5..=9) => six_and_five_pm(),
        (18, 10..=14) => six_and_ten_pm(),
        (18, 15..=19) => six_and_a_quarter_pm(),
        (18, 20..=24) => six_and_twenty_pm(),
        (18, 25..=29) => six_and_twenty_five_pm(),
        (18, 30..=34) => six_and_half_pm(),
        (18, 35..=39) => seven_less_twenty_five_pm(),
        (18, 40..=44) => seven_less_twenty_pm(),
        (18, 45..=49) => seven_less_a_quarter_pm(),
        (18, 50..=54) => seven_less_ten_pm(),
        (18, 55..=59) => seven_less_five_pm(),
        _ => Vec::new(),
    }
}
