# Fort

Fort is proc macro attribute crate for Bastion.

## Usage
```toml
[dependencies]
fort = "0.2"
bastion = "0.3.*"
```

You can directly use fort to load work onto the root supervisor with:
```rust
#[fort::root]
async fn main(_: BastionContext) -> Result<(), ()> {
    println!("Running in Bastion runtime!");
    Ok(())
}
```

Make your program fault-tolerant with `fort`:
```rust
#[fort::root]
async fn main(_: BastionContext) -> Result<(), ()> {
    loop {
        println!("Undying main!");
        panic!("Error")
    }
}
```

You want to spawn multiple process
```rust
#[fort::root(redundancy = 10)]
async fn main(_: BastionContext) -> Result<(), ()> {
    loop {
        println!("Undying main!");
        panic!("Error")
    }
}
```

# Example TCP Server

```rust
use std::io::Write;
use std::net::TcpListener;

#[fort::root]
async fn main(_: BastionContext) -> Result<(), ()> {
    let listener = TcpListener::bind("127.0.0.1:2278").unwrap();
    println!("TCP server started at 127.0.0.1:2278");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let mut stream = stream.unwrap();
            stream.write_all(b"Hello World\r\n").unwrap();
            panic!("Fail in thread!");
        });
        panic!("Fail in event loop");
    }

    Ok(())
}
```
