use apple_frameworks::{NSArray, NSDictionary};

fn main() {
    let arr = NSArray![1, 2, 3];

    for i in &arr {
        println!("{}", i);
    }
}
