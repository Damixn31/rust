use dotenv::dotenv;
use run::run_executable::executable;

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
    pub mod get_audio;
    pub mod play_audio;
}

static mut EXECUTED: bool = false;

fn main() {
    dotenv().ok();

    unsafe {
        if !EXECUTED {
            executable();
            EXECUTED = true;
        }
    }
}
