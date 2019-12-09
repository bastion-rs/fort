use bastion::prelude::*;

#[fort::root(redundancy = 10)]
async fn main(_: BastionContext) -> Result<(), ()> {
    loop {
        println!("Undying main!");
        panic!("Error")
    }
}
