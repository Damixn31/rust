use ssh2::Session;
use std::error::Error;
use std::net::TcpStream;

pub fn conn_tcp(user: &str, host: &str, port: u16) -> Result<Session, Box<dyn Error>> {
    let tcp = TcpStream::connect((host, port))?;

    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    session.userauth_agent(user)?;

    Ok(session)
}
