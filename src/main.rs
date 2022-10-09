use std::time::Instant;
use text_io::read;

fn main() {
    print!("Please enter a number for n: ");
    let n: i32 = read!();
    let now = Instant::now();
    let square = n * n;
    let later = Instant::now();
    println!("n squared = {}", square);
    print!("Time elapsed: {:?}", later.duration_since(now))
}
