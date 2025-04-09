use remotro::Remotro;

#[tokio::main]
async fn main() {
    let remotro = Remotro::host("127.0.0.1", 34143).await.unwrap();
    println!("Remotro hosted on 127.0.0.1:34143");
}
