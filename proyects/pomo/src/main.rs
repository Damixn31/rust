use pomo::clock::run::pomodoro_run;

fn main() {
    let w_duration = 1;
    let s_break = 5;
    let l_break = 15;
    let cycles = 4;

    pomodoro_run(w_duration, s_break, l_break, cycles);
}
