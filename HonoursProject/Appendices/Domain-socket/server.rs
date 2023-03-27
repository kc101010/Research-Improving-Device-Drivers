use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{BufRead, BufReader};
use common::SERVER_NAME;

mod common;

fn handle_client(stream: UnixStream){
    let stream = BufReader::new(stream);

    for line in stream.lines() {
        println!("{}", line.unwrap());
    }
}

fn main() -> std::io::Result<()> {
    let listener = UnixListener::bind(SERVER_NAME).unwrap();

    for stream in listener.incoming(){

        match stream{
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }

            Err(err) =>{
                println!("Error: {}", err);
                break;
            }

        }
    }

    Ok(())
}