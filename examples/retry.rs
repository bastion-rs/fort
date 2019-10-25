#[fort::root(retry = "2")]
fn main() {
    loop {
        println!("Undying main!");
        panic!("Error")
    }
}
