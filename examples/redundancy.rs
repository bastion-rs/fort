use bastion::prelude::*;

#[fort::root(redundancy = 2)]
async fn main(_: BastionContext) -> Result<(), ()> {
    println!("Apply redundancy, default is 1");
    Ok(())
}
