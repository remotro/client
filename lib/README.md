This crate provides a method to interface with Balatro when paired with the mod at <https://github.com/remotro/mod>

To create a new listener on port 34143 (Default in mod) you can use:
```rust
use remotro::Remotro;
fn main() {
    let mut remotro = Remotro::host("0.0.0.0", 34143).await.expect("Socket is not available");
    loop {
        let mut balatro = match remotro.accept().await {
            Ok(b) => b,
            Err(e) => println!("Connection failed: {e}")
        };
    }
}
```
Which will attempt to open a port and wait for the mod to connect to it