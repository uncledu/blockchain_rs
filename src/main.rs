pub mod block;
pub mod blockchain;
pub type Result<T> = std::result::Result<T, failure::Error>;
use std::thread::sleep;
use std::time::Duration;
fn main() -> Result<()> {
    let mut bc = blockchain::Blockchain::new();
    sleep(Duration::from_millis(10));
    bc.add_block(String::from("Send 1 BTC to Ivan"))?;
    sleep(Duration::from_millis(30));
    bc.add_block(String::from("Send 2 more BTC to Ivan"))?;

    println!("Blockchain: {:#?}", bc);
    Ok(())
}
