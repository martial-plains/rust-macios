#[macro_use]
extern crate apple_frameworks;

fn main() {
    let nsarr = NSArray![1, 2, 3];

    for item in &nsarr {
        println!("{}", item);
    }
}
