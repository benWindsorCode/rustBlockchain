use blockchain::blockchain::Blockchain;

fn main() {
    println!("Hello, world!");
    let chain = Blockchain::new();
    println!("{:?}", chain);
}
