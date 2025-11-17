This crate provides a method to interact with Balatro when used with the mod at <https://github.com/remotro/mod>

Sample Implementation:
```rust
use remotro::Remotro;
use remotro::balatro::CurrentScreen::*;

fn main() {
    let mut remotro = Remotro::host("0.0.0.0", 34143).await.expect("Socket is not available");
    loop {
        let mut balatro = match remotro.accept().await {
            Ok(b) => b,
            Err(e) => println!("Connection failed: {e}")
        };
        
        loop {
            match balatro.screen().await {
                Ok(screen) => match screen {
                    Menu(menu) => {
                        /* Menu handler */
                    }
                    /*...*/
                    GameOver(game) => {
                        /* Game Over handler */
                    }
                }
                Err(e) => {
                    println!("{e}");
                    break; // Goes back to listening for connections
                }
            }
        }
    }
}
```
This code will attempt to open a port and wait for the mod to connect to it, then continually matches the current screen, running the code specified for each screen
