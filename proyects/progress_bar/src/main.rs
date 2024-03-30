use std::{thread, time::Duration};

use console::{style, Style};

fn clear_console() {
    print!("\x1B[2J\x1B[H");
}

fn main() {
    let total_frames = 30;
    let total_bytes = 1000;
    let mut _progress = 0;

    for frame in 0..total_frames {
        clear_console();
        print!(
            "{} Progress... {}",
            spinner(frame),
            progress_bar(_progress, total_bytes)
        );
        thread::sleep(Duration::from_millis(50));
        _progress += total_bytes / total_frames;
    }
    //print!("\nReady!");
}

fn spinner(frame: usize) -> String {
    //let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let frames: [(&str, Style); 10] = [
        ("⠋", Style::new().magenta()),
        ("⠙", Style::new().red()),
        ("⠹", Style::new().yellow()),
        ("⠸", Style::new().green()),
        ("⠼", Style::new().white()),
        ("⠴", Style::new().cyan()),
        ("⠦", Style::new().blue()),
        ("⠧", Style::new().black()),
        ("⠇", Style::new().red()),
        ("⠏", Style::new().magenta()),
    ];

    let (frame_text, frame_style) = &frames[frame % frames.len()];
    frame_style.apply_to(*frame_text).to_string()
}

fn progress_bar(progress: usize, total: usize) -> String {
    let width = 20;
    let progress_percentage = (progress * 100) / total;
    let progress_bar_width = (progress_percentage * width) / 100;
    let mut bar = String::new();

    for i in 0..width {
        if i <= progress_bar_width {
            bar.push_str(&style("█").cyan().to_string());
        } else {
            bar.push_str(&style("░").cyan().to_string());
        }
    }
    format!("[{} {}]", bar, progress_percentage)
}
