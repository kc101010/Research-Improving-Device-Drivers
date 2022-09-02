use std::os::unix::net::{UnixStream, UnixListener};
use std::io::prelude::*;
use std::io::{stdin};
use common::SERVER_NAME;

mod common;

fn main() -> std::io::Result<()> {
    let listener = UnixListener::bind(SERVER_NAME);
    let mut server = UnixStream::connect(SERVER_NAME)?;
    let mut message = String::new();

    stdin().read_line(&mut message)
        .ok()
        .expect("Failed to read line");

    server.write_all(b"{message}")?;

    Ok(())

}