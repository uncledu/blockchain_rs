pub mod block;
pub mod blockchain;
pub type Result<T> = std::result::Result<T, failure::Error>;

fn main() {
    println!("Hello, world!");
}
