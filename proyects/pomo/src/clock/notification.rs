use notify_rust::Notification;

pub fn notify(message: &str) {
    Notification::new()
        .summary("Pomodoro Timer")
        .body(message)
        .show()
        .unwrap();
}
