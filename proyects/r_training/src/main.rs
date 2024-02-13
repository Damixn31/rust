use dotenv::dotenv;
use run::run_executable::executable;
//use load_exercises::load_weekly_exercises::load_weekly_exercises;
use timing::run_timer::timmer;

mod timing {
    pub mod run_timer;
}

mod exercises {
    pub mod exercise;
}

mod load_exercises {
    pub mod load_weekly_exercises;
}
mod run {
    pub mod run_executable;
}

mod audio {
    pub mod play_audio;
}

fn main() {
    dotenv().ok();
    executable();
}
