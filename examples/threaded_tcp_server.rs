use std::io::Write;
use std::net::TcpListener;
use std::thread;

#[fort::root]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:2278").unwrap();
    println!("TCP server started at 127.0.0.1:2278");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            stream.write(b"Hello World\r\n").unwrap();
            panic!("Fail in thread!");
        });
        panic!("Fail in event loop");
    }
}
