use std::{net::{TcpStream, TcpListener}, io::{Result, BufReader, BufRead, Write}};
use base64::engine::general_purpose;
use sha1::{Sha1, Digest};
use base64::Engine as _;

fn main() -> Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:5678")?;

    for stream in listener.incoming() {
        handle_connection(stream?);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> () {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buf = String::new();
    let _ = buf_reader.read_line(&mut buf).unwrap();

    println!("{buf}");

    let mut headers: Vec<String> = Vec::new();

    headers.push(String::from("HTTP/1.1 101 Switching Protocols"));
    headers.push(String::from("Upgrade: websocket"));
    headers.push(String::from("Connection: Upgrade"));

    for line in buf_reader.lines().map(|l| l.unwrap()).take_while(|l| !l.is_empty()) {
        let mut tokens = line.split(": ");

        if let Some(key) = tokens.next() {
            if key == "Sec-WebSocket-Key" {
                let value = tokens.next().unwrap();
                // println!("pre-hash # bytes: {}", value.len());
                let guid = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
                let input = String::from(value) + guid;
                // println!("pre-hash input: {input}");

                let mut hasher = Sha1::new();
                hasher.update(input.as_bytes());
                let hash = hasher.finalize();

                // println!("hashed: {:#?}", hash);

                let b64_encoded = general_purpose::STANDARD.encode(hash);
                // println!("base64: {b64_encoded} length: {}", b64_encoded.len());
                let res_header = format!("Sec-WebSocket-Accept: {b64_encoded}\r\n\r\n");
                headers.push(res_header);
                break;
            }
        }
    }

    for header in headers.clone() {
        println!("{header}");
    }

    if buf.starts_with("GET /chat HTTP/") {
        println!("we tryna websocket :)");
        // println!("{:#?}", headers.join("\r\n"));
        stream.write_all(headers.join("\r\n").as_bytes()).unwrap();
        println!("lol");
    }
}