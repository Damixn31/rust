use notify_rust::Notification;

pub fn send_notification(message: &str) {
    if let Err(e) = Notification::new()
        .summary("Alerta de seguridad")
        .body(message)
        .icon("security-high")
        .show()
    {
        eprintln!("Error al enviar la notificacion: {}", e);
    }
}
