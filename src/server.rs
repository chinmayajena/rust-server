//Everything inside a separate file is implicitly referenced as module
//Everything inside a module(mod) is private

use std::{ io::Read, net::TcpListener };
use crate::http::Request;

pub struct Server {
    addr: String,
}

//[u8;5] means an array of 5 unsigned int of 8 bit. This means 0 to 255
//if you need to pass dynamic array then better to pass reference &[u8].
fn arr(a: &[u8]) {}

//Self is alias of the Struct name. "self" is like this in java
impl Server {
    pub fn new(address: String) -> Self {
        Server {
            addr: address,
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        //&self.addr is sending the reference instead of moving the addr variable;
        //unwrap works like promise. if ok then send the result to listener variable otherwise terminate program.
        let listener = TcpListener::bind(&self.addr).unwrap();

        //We need infinity loop and check in every loop for any connection
        //loop is infinite loop. Loop can be named as well. use 'name_of_loop: loop
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            //Here we used [..] to convert the [u8,1024] into [u8]
                            match Request::try_from(&buffer[..]) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Failed to parse the request: {}", e);
                                }
                            }
                        }

                        Err(e) => {
                            println!("Failed to read buffer!");
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}
