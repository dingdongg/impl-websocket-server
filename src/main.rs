use std::{net::{TcpStream, TcpListener}, io::{Result, Read, BufReader, BufRead}};

fn main() -> Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:5678")?;

    for stream in listener.incoming() {
        handle_connection(&mut stream?);
    }

    Ok(())
}

fn handle_connection(stream: &TcpStream) -> () {
    println!("HELLO WORLD");
    let mut buf_reader = BufReader::new(stream);
    let mut buf = String::new();
    let _ = buf_reader.read_line(&mut buf).unwrap();

    if buf.starts_with("GET /chat HTTP/") {
        println!("we tryna websocket :)");
    }
    
    // for line in buf_reader.lines() {
    //     println!("{:#?}", line.unwrap());
    // }

    println!("{:#?}", buf);
    println!("---");
}