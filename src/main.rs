use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;

fn handle_connection(mut stream: TcpStream, mut table: &mut HashMap<String, Vec<u8>>) -> Result<(), Box<dyn Error>> {
    loop {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        let msg_quit = b"quit";
        let msg_set = b"set";
        let msg_get = b"get";

        if (buffer.starts_with(msg_quit)) {
            let response = "QUIT\r\n";
            stream.write(response.as_bytes())?;
            stream.flush()?;
            break;
        } else if (buffer.starts_with(msg_set)) {
            let row = String::from_utf8_lossy(&buffer[..]);
            let mut data = row.split_whitespace().collect::<Vec<&str>>();
            let key = data[1];
            let value = data[2..].join(" ");
            table.insert(String::from(key), Vec::from(value.as_bytes()));

            let response = "STORED\r\n";
            stream.write(response.as_bytes())?;
            stream.flush()?;
        } else if (buffer.starts_with(msg_get)) {
            let row = String::from_utf8_lossy(&buffer[..]);
            let mut data = row.split_whitespace().collect::<Vec<&str>>();
            let key = data[1];
            let value = table.get(key);
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

    let table: Rc<RefCell<HashMap<String, Vec<u8>>>> =
      Rc::new(RefCell::new(HashMap::new()));

    for stream in listener.incoming() {
        let stream = stream?;

        println!("Connection established!");
        handle_connection(stream, &mut table.clone().borrow_mut())?;
    }

    Ok(())
}
