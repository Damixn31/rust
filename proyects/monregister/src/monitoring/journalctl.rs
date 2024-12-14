use std::{
    fs::File,
    io::BufReader,
    os::fd::{FromRawFd, IntoRawFd, RawFd},
    process::{Child, Command, Stdio},
};

pub fn spawn_journalctl_process() -> Result<std::process::Child, std::io::Error> {
    Command::new("journalctl")
        .arg("-f")
        .arg("_SYSTEMD_UNIT=sshd.service") // modificar si el servicio es ssh
        .stdout(Stdio::piped())
        .spawn()
}

pub fn get_stdout_reader(process: Child) -> Result<BufReader<File>, std::io::Error> {
    let raw_fd: RawFd = process
        .stdout
        .expect("Failed to capture stdout")
        .into_raw_fd();
    let file = unsafe { std::fs::File::from_raw_fd(raw_fd) };
    Ok(BufReader::new(file))
}
