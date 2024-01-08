//use colored::*;
use square::get_position::get_position_highlighted;
//use std::thread;
//use std::time::{Duration, SystemTime};

mod square {
    pub mod get_position;
    pub mod print_square;
}

mod twelve {
    pub mod it_is_a_quarter_past_twelve;
    pub mod it_is_five_to_twelve_o_clock;
    pub mod it_is_half_past_twelve;
    pub mod it_is_ten_to_twelve_o_clock;
    pub mod it_is_twelve;
    pub mod it_is_twelve_and_five;
    pub mod it_is_twelve_and_ten;
    pub mod it_is_twelve_less_one_fourth;
    pub mod it_is_twelve_less_twenty_five;
    pub mod it_is_twenty_five_past_twelve;
    pub mod it_is_twenty_past_twelve;
    pub mod it_is_twenty_to_one_o_clock;
}

use square::print_square::print_square;

fn main() {
    let hour_current = chrono::Local::now().format("%H:%M:%S").to_string();

    let position = get_position_highlighted(&hour_current);
    print_square(&position, &hour_current);
}
