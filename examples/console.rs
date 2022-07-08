use rust_macios::foundation::macros::ns_array;

fn main() {
    let arr = ns_array! {1, 2, 3, 4, 5};

    println!("{}", arr);
    println!("Still prints");
}
