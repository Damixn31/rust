use std::{
    //  fs::File,
    //  io::BufReader,
    //  os::fd::{FromRawFd, IntoRawFd, RawFd},
    io::BufReader,
    process::{Child, ChildStdout, Command, Stdio},
};

pub fn spawn_journalctl_process(service: &str) -> Result<std::process::Child, std::io::Error> {
    Command::new("journalctl")
        .arg("-u")
        .arg(service) // modificar si el servicio es ssh
        .arg("-f")
        .stdout(Stdio::piped())
        .spawn()
}

// verificar si esta trabajando bien los procesos
pub fn get_stdout_reader(process: Child) -> BufReader<ChildStdout> {
    BufReader::new(process.stdout.expect("Failed to capture stdout"))
}

//pub fn get_stdout_reader(process: Child) -> Result<BufReader<File>, std::io::Error> {
//    let raw_fd: RawFd = process
//        .stdout
//        .expect("Failed to capture stdout")
//        .into_raw_fd();
//    let file = unsafe { std::fs::File::from_raw_fd(raw_fd) };
//    Ok(BufReader::new(file))
//}
