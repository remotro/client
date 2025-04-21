use remotro::{Remotro, balatro::Screen};
use log;

#[tokio::main]
async fn main() {
    env_logger::init(); // Initialize the logger

    let mut remotro = Remotro::host("127.0.0.1", 34143).await.unwrap();
    log::info!("Remotro hosted on 127.0.0.1:34143");
    loop {
        let mut balatro = remotro.accept().await.unwrap();
        // let screen = balatro.screen().await.unwrap();
        // match screen {
        //     Screen::Menu(menu) => {
        //         menu.new_run().await.unwrap();
        //     }
        // }
        loop {}
    }
}
