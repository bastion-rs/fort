# Fort

Fort is proc macro attribute crate for Bastion.

You can directly use fort to load work onto the root supervisor with:
```rust
#[fort::root]
fn main() {
    println!("Running in Bastion runtime!");
}
```
