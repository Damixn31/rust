use mutex::Mutex;
use std::sync::Arc;
//use colored::*;
use square::get_position::get_position_highlighted;
//use std::thread;
//use std::time::{Duration, SystemTime};
//use tokio::time::{interval, Duration};

mod square {
    pub mod get_position;
    pub mod print_square;
    pub mod print_square_auto;
}

mod helpers {
    pub mod convert_to_usize_tupla;
}

mod twelve {
    pub mod it_is_a_quarter_past_twelve;
    pub mod it_is_five_to_twelve_o_clock;
    pub mod it_is_half_past_twelve;
    pub mod it_is_ten_to_twelve_o_clock;
    pub mod it_is_twelve_am;
    pub mod it_is_twelve_and_five_am;
    pub mod it_is_twelve_and_five_pm;
    pub mod it_is_twelve_and_ten_am;
    pub mod it_is_twelve_and_ten_pm;
    pub mod it_is_twelve_less_one_fourth;
    pub mod it_is_twelve_less_twenty_five;
    pub mod it_is_twelve_pm;
    pub mod it_is_twenty_five_past_twelve;
    pub mod it_is_twenty_past_twelve;
    pub mod it_is_twenty_to_one_o_clock;
}

mod one {
    pub mod one_am;
    pub mod one_pm;
}

use square::print_square::print_square;
use square::print_square_auto::print_square_auto;

#[tokio::main]
async fn main() {
    //let hour_current = chrono::Local::now().format("%H:%M:%S").to_string();

    //let position = get_position_highlighted(&hour_current);
    //print_square(&position, &hour_current);
    let position_highlighted = Arc::new(Mutex::new(Vec::new()));

    print_square_auto(position_highlighted.clone()).await;
}
