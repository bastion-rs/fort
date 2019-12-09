use bastion::prelude::*;

#[fort::root]
async fn main(_: BastionContext) -> Result<(), ()> {
    println!("Running in Bastion runtime!");
    Ok(())
}
