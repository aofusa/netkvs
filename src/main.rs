use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

fn handle_connection(mut stream: TcpStream, table: &Arc<RwLock<HashMap<String, Vec<u8>>>>) -> Result<(), Box<dyn Error + '_>> {
    loop {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        let msg_quit = b"quit";
        let msg_set = b"set";
        let msg_get = b"get";

        if buffer.starts_with(msg_quit) {
            let response = "QUIT\r\n";
            stream.write(response.as_bytes())?;
            stream.flush()?;
            break;
        } else if buffer.starts_with(msg_set) {
            let row = String::from_utf8_lossy(&buffer[..]);
            let data = row.split_whitespace().collect::<Vec<&str>>();
            let key = data[1];
            let value = data[2..].join(" ");
            table.write()?.insert(String::from(key), Vec::from(value.as_bytes()));

            let response = "STORED\r\n";
            stream.write(response.as_bytes())?;
            stream.flush()?;
        } else if buffer.starts_with(msg_get) {
            let row = String::from_utf8_lossy(&buffer[..]);
            let data = row.split_whitespace().collect::<Vec<&str>>();
            let key = data[1];
            let t = table.read()?;
            let value = t.get(key);
            let response = if value.is_some() {
                value.ok_or("get value error")?
            } else {
                &vec![0; 0]
            };
            stream.write(response)?;
            stream.write("\r\nEND\r\n".as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let host = "127.0.0.1:11211";
    let listener = TcpListener::bind(host)?;
    println!("Server listen on {}", host);

    let table: Arc<RwLock<HashMap<String, Vec<u8>>>> =
      Arc::new(RwLock::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream?;
        let t = table.clone();

        println!("Connection established!");
        thread::spawn(move || {
            handle_connection(stream, &t).unwrap();
        });
    }

    Ok(())
}
