use mutex::Mutex;
use square::get_position::get_position_highlighted;
use std::sync::Arc;

mod square {
    pub mod get_position;
    pub mod print_square;
    pub mod print_square_auto;
}

mod helpers {
    pub mod convert_to_usize_tupla;
}

mod twelve {
    pub mod twelve_am;
    pub mod twelve_pm;
}

mod one {
    pub mod one_am;
    pub mod one_pm;
}

mod two {
    pub mod two_am;
    pub mod two_pm;
}

mod three {
    pub mod three_am;
    pub mod three_pm;
}

mod four {
    pub mod four_am;
    pub mod four_pm;
}

mod five {
    pub mod five_am;
    pub mod five_pm;
}

mod six {
    pub mod six_am;
    pub mod six_pm;
}
mod seven {
    pub mod seven_am;
    pub mod seven_pm;
}

mod eight {
    pub mod eight_am;
    pub mod eight_pm;
}

mod nine {
    pub mod nine_am;
    pub mod nine_pm;
}

mod ten {
    pub mod ten_am;
    pub mod ten_pm;
}

mod eleven {
    pub mod eleven_am;
    pub mod eleven_pm;
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
