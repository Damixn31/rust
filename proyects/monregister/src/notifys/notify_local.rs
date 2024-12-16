use notify_rust::Notification;

pub fn send_notification(ip: &str, attempts: u32) {
    let title = "Alerta de seguridad: SSH";
    let message = format!(
        "IP: {} ha fallado {} veces al intentar conectarse.",
        ip, attempts
    );

    if let Err(e) = Notification::new()
        .summary(title)
        .body(&message)
        .icon("security-high")
        .show()
    {
        eprintln!("Error al enviar la notificacion: {}", e);
    }
}
