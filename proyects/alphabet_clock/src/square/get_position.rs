use std::usize;

use chrono::{Local, Timelike};

use crate::eight::eight_am::{
    eight_am, eight_and_a_quarter_am, eight_and_five_am, eight_and_half_am, eight_and_ten_am,
    eight_and_twenty_am, eight_and_twenty_five_am, nine_less_a_quarter_am, nine_less_five_am,
    nine_less_ten_am, nine_less_twenty_am, nine_less_twenty_five_am,
};
use crate::eight::eight_pm::{
    eight_and_a_quarter_pm, eight_and_five_pm, eight_and_half_pm, eight_and_ten_pm,
    eight_and_twenty_five_pm, eight_and_twenty_pm, eight_pm, nine_less_a_quarter_pm,
    nine_less_five_pm, nine_less_ten_pm, nine_less_twenty_five_pm, nine_less_twenty_pm,
};
use crate::eleven::eleven_am::{
    eleven_am, eleven_and_a_quarter_am, eleven_and_five_am, eleven_and_half_am, eleven_and_ten_am,
    eleven_and_twenty_am, eleven_and_twenty_five_am, twelve_less_a_quarter_am, twelve_less_five_am,
    twelve_less_ten_am, twelve_less_twenty_am, twelve_less_twenty_five_am,
};
use crate::eleven::eleven_pm::{
    eleven_and_a_quarter_pm, eleven_and_five_pm, eleven_and_half_pm, eleven_and_ten_pm,
    eleven_and_twenty_five_pm, eleven_and_twenty_pm, eleven_pm, twelve_less_a_quarter_pm,
    twelve_less_five_pm, twelve_less_ten_pm, twelve_less_twenty_five_pm, twelve_less_twenty_pm,
};
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
use crate::nine::nine_am::{
    nine_am, nine_and_a_quarter_am, nine_and_five_am, nine_and_half_am, nine_and_ten_am,
    nine_and_twenty_am, nine_and_twenty_five_am, ten_less_a_quarter_am, ten_less_five_am,
    ten_less_ten_am, ten_less_twenty_am, ten_less_twenty_five_am,
};
use crate::nine::nine_pm::{
    nine_and_a_quarter_pm, nine_and_five_pm, nine_and_half_pm, nine_and_ten_pm,
    nine_and_twenty_five_pm, nine_and_twenty_pm, nine_pm, ten_less_a_quarter_pm, ten_less_five_pm,
    ten_less_ten_pm, ten_less_twenty_five_pm, ten_less_twenty_pm,
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

use crate::seven::seven_am::{
    eight_less_a_quarter_am, eight_less_five_am, eight_less_ten_am, eight_less_twenty_am,
    eight_less_twenty_five_am, seven_am, seven_and_a_quarter_am, seven_and_five_am,
    seven_and_half_am, seven_and_ten_am, seven_and_twenty_five_am,
};
use crate::seven::seven_pm::{
    eight_less_a_quarter_pm, eight_less_five_pm, eight_less_ten_pm, eight_less_twenty_five_pm,
    eight_less_twenty_pm, seven_and_a_quarter_pm, seven_and_five_pm, seven_and_half_pm,
    seven_and_ten_pm, seven_and_twenty_five_pm, seven_and_twenty_pm, seven_pm,
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
use crate::ten::ten_am::{
    eleven_less_a_quarter_am, eleven_less_five_am, eleven_less_ten_am, eleven_less_twenty_am,
    eleven_less_twenty_five_am, ten_am, ten_and_a_quarter_am, ten_and_five_am, ten_and_half_am,
    ten_and_ten_am, ten_and_twenty_am, ten_and_twenty_five_am,
};
use crate::ten::ten_pm::{
    eleven_less_a_quarter_pm, eleven_less_five_pm, eleven_less_ten_pm, eleven_less_twenty_five_pm,
    eleven_less_twenty_pm, ten_and_a_quarter_pm, ten_and_five_pm, ten_and_half_pm, ten_and_ten_pm,
    ten_and_twenty_five_pm, ten_and_twenty_pm, ten_pm,
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

        // ------------- 7
        (7, 0..=4) => seven_am(),
        (7, 5..=9) => seven_and_five_am(),
        (7, 10..=14) => seven_and_ten_am(),
        (7, 15..=19) => seven_and_a_quarter_am(),
        (7, 20..=24) => seven_and_ten_am(),
        (7, 25..=29) => seven_and_twenty_five_am(),
        (7, 30..=34) => seven_and_half_am(),
        (7, 35..=39) => eight_less_twenty_five_am(),
        (7, 40..=44) => eight_less_twenty_am(),
        (7, 45..=49) => eight_less_a_quarter_am(),
        (7, 50..=54) => eight_less_ten_am(),
        (7, 55..=59) => eight_less_five_am(),

        // -------------- 8
        (8, 0..=4) => eight_am(),
        (8, 5..=9) => eight_and_five_am(),
        (8, 10..=14) => eight_and_ten_am(),
        (8, 15..=19) => eight_and_a_quarter_am(),
        (8, 20..=24) => eight_and_twenty_am(),
        (8, 25..=29) => eight_and_twenty_five_am(),
        (8, 30..=34) => eight_and_half_am(),
        (8, 35..=39) => nine_less_twenty_five_am(),
        (8, 40..=44) => nine_less_twenty_am(),
        (8, 45..=49) => nine_less_a_quarter_am(),
        (8, 50..=54) => nine_less_ten_am(),
        (8, 55..=59) => nine_less_five_am(),

        // ------------- 9
        (9, 0..=4) => nine_am(),
        (9, 5..=9) => nine_and_five_am(),
        (9, 10..=14) => nine_and_ten_am(),
        (9, 15..=19) => nine_and_a_quarter_am(),
        (9, 20..=24) => nine_and_twenty_am(),
        (9, 25..=29) => nine_and_twenty_five_am(),
        (9, 30..=34) => nine_and_half_am(),
        (9, 35..=39) => ten_less_twenty_five_am(),
        (9, 40..=44) => ten_less_twenty_am(),
        (9, 45..=49) => ten_less_a_quarter_am(),
        (9, 50..=54) => ten_less_ten_am(),
        (9, 55..=59) => ten_less_five_am(),

        // ------------- 10
        (10, 0..=4) => ten_am(),
        (10, 5..=9) => ten_and_five_am(),
        (10, 10..=14) => ten_and_ten_am(),
        (10, 15..=19) => ten_and_a_quarter_am(),
        (10, 20..=24) => ten_and_twenty_am(),
        (10, 25..=29) => ten_and_twenty_five_am(),
        (10, 30..=34) => ten_and_half_am(),
        (10, 35..=39) => eleven_less_twenty_five_am(),
        (10, 40..=44) => eleven_less_twenty_am(),
        (10, 45..=49) => eleven_less_a_quarter_am(),
        (10, 50..=54) => eleven_less_ten_am(),
        (10, 55..=59) => eleven_less_five_am(),

        // ------------- 11
        (11, 0..=4) => eleven_am(),
        (11, 5..=9) => eleven_and_five_am(),
        (11, 10..=14) => eleven_and_ten_am(),
        (11, 15..=19) => eleven_and_a_quarter_am(),
        (11, 20..=24) => eleven_and_twenty_am(),
        (11, 25..=29) => eleven_and_twenty_five_am(),
        (11, 30..=34) => eleven_and_half_am(),
        (11, 35..=39) => twelve_less_twenty_five_am(),
        (11, 40..=44) => twelve_less_twenty_am(),
        (11, 45..=49) => twelve_less_a_quarter_am(),
        (11, 50..=54) => twelve_less_ten_am(),
        (11, 55..=59) => twelve_less_five_am(),

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
        (14, 0..=4) => two_pm(),
        (14, 5..=9) => two_and_five_pm(),
        (14, 10..=14) => two_and_ten_pm(),
        (14, 15..=19) => two_and_a_quarter_pm(),
        (14, 20..=24) => two_and_twenty_pm(),
        (14, 25..=29) => two_and_twenty_five_pm(),
        (14, 30..=34) => two_and_half_pm(),
        (14, 35..=39) => three_less_twenty_five_pm(),
        (14, 40..=44) => three_less_twenty_pm(),
        (14, 45..=49) => three_less_a_quarter_pm(),
        (14, 50..=54) => three_less_ten_pm(),
        (14, 55..=59) => three_less_five_pm(),

        // ------- THREE
        (15, 0..=4) => three_pm(),
        (15, 5..=9) => three_and_five_pm(),
        (15, 10..=14) => three_and_ten_pm(),
        (15, 15..=19) => three_and_a_quarter_pm(),
        (15, 20..=24) => three_and_twenty_pm(),
        (15, 25..=29) => three_and_twenty_five_pm(),
        (15, 30..=34) => three_and_half_pm(),
        (15, 35..=39) => four_less_twenty_five_pm(),
        (15, 40..=44) => four_less_twenty_pm(),
        (15, 45..=49) => four_less_twenty_five_pm(),
        (15, 50..=54) => four_less_ten_pm(),
        (15, 55..=59) => four_less_five_pm(),

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

        // -------- SEVEN
        (19, 0..=4) => seven_pm(),
        (19, 5..=9) => seven_and_five_pm(),
        (19, 10..=14) => seven_and_ten_pm(),
        (19, 15..=19) => seven_and_a_quarter_pm(),
        (19, 20..=24) => seven_and_twenty_pm(),
        (19, 25..=29) => seven_and_twenty_five_pm(),
        (19, 30..=34) => seven_and_half_pm(),
        (19, 35..=39) => eight_less_twenty_five_pm(),
        (19, 40..=44) => eight_less_twenty_pm(),
        (19, 45..=49) => eight_less_a_quarter_pm(),
        (19, 50..=54) => eight_less_ten_pm(),
        (19, 55..=59) => eight_less_five_pm(),

        // ---------- EIGHT
        (20, 0..=4) => eight_pm(),
        (20, 5..=9) => eight_and_five_pm(),
        (20, 10..=14) => eight_and_ten_pm(),
        (20, 15..=19) => eight_and_a_quarter_pm(),
        (20, 20..=24) => eight_and_twenty_pm(),
        (20, 25..=29) => eight_and_twenty_five_pm(),
        (20, 30..=34) => eight_and_half_pm(),
        (20, 35..=39) => nine_less_twenty_five_pm(),
        (20, 40..=44) => nine_less_twenty_pm(),
        (20, 45..=49) => nine_less_a_quarter_pm(),
        (20, 50..=54) => nine_less_ten_pm(),
        (20, 55..=59) => nine_less_five_pm(),

        // -------------- NINE
        (21, 0..=4) => nine_pm(),
        (21, 5..=9) => nine_and_five_pm(),
        (21, 10..=14) => nine_and_ten_pm(),
        (21, 15..=19) => nine_and_a_quarter_pm(),
        (21, 20..=24) => nine_and_twenty_pm(),
        (21, 25..=29) => nine_and_twenty_five_pm(),
        (21, 30..=34) => nine_and_half_pm(),
        (21, 35..=39) => ten_less_twenty_five_pm(),
        (21, 40..=44) => ten_less_twenty_pm(),
        (21, 45..=49) => ten_less_a_quarter_pm(),
        (21, 50..=54) => ten_less_ten_pm(),
        (21, 55..=59) => ten_less_five_pm(),

        // -------------- TEN
        (22, 0..=4) => ten_pm(),
        (22, 5..=9) => ten_and_five_pm(),
        (22, 10..=14) => ten_and_ten_pm(),
        (22, 15..=19) => ten_and_a_quarter_pm(),
        (22, 20..=24) => ten_and_twenty_pm(),
        (22, 25..=29) => ten_and_twenty_five_pm(),
        (22, 30..=34) => ten_and_half_pm(),
        (22, 35..=39) => eleven_less_twenty_five_pm(),
        (22, 40..=44) => eleven_less_twenty_pm(),
        (22, 45..=49) => eleven_less_a_quarter_pm(),
        (22, 50..=54) => eleven_less_ten_pm(),
        (22, 55..=59) => eleven_less_five_pm(),

        // -------------- ELEVEN
        (23, 0..=4) => eleven_pm(),
        (23, 5..=9) => eleven_and_five_pm(),
        (23, 10..=14) => eleven_and_ten_pm(),
        (23, 15..=19) => eleven_and_a_quarter_pm(),
        (23, 20..=24) => eleven_and_twenty_pm(),
        (23, 25..=29) => eleven_and_twenty_five_pm(),
        (23, 30..=34) => eleven_and_half_pm(),
        (23, 35..=39) => twelve_less_twenty_five_pm(),
        (23, 40..=44) => twelve_less_twenty_pm(),
        (23, 45..=49) => twelve_less_a_quarter_pm(),
        (23, 50..=54) => twelve_less_ten_pm(),
        (23, 55..=59) => twelve_less_five_pm(),

        _ => Vec::new(),
    }
}
