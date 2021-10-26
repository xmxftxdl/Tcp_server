use std::io::{Error, Read};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    //Create buffer to hold the come in data
    let mut buffer = [0; 1024];
    //Pass the buffer to Stream.read. Read bytes from TcpServer and put in the buffer
    stream.read(&mut buffer).unwrap();
    //Convert the bytes in the buffer to a string and print that string.
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
fn main() -> Result<(), Error> {
    //Create a listener for local address 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //Read income when the connection still alive
    for stream in listener.incoming() {
        //Error handling. check if the stream is null
        match stream {
            Ok(_stream) => {
                //If the connection is successfully, 
                for stream in listener.incoming() {
                    //create stream and if steam has some error, unwrap()function will be called and program will be terminated
                    let stream = stream.unwrap();
                    println!("Connection established!");
                    //Call the handle_connection function and print the result
                    handle_connection(stream);
                }
            }
            //
            Err(_e) => {
                //error handling.
                println!("Connection Failed!");
            }
        }
    }
    Ok(())
}