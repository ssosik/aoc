struct Pair{
    x: PairItem,
    y: PairItem,
}

enum PairItem {
    Num(u8),
    Pair(Box<Pair>),
}

fn main() {
    println!("Hello, world!");
}
