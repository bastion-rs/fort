# Fort

Fort is proc macro attribute crate for Bastion.

You can directly use fort to load work onto the root supervisor with:
```rust
#[fort::root]
fn main() {
    println!("Running in Bastion runtime!");
}
```

# Example TCP Server

```rust
use std::io::Write;
use std::net::TcpListener;

#[fort::root]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:2278").unwrap();
    println!("TCP server started at 127.0.0.1:2278");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.write(b"Hello World\r\n").unwrap();
        panic!("Fail here!");
    }
}
```
