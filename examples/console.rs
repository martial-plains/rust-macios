use rust_macios::foundation::NSOrthography;

fn main() {
    let arr = NSOrthography::default_orthography_for_language("en");

    println!("{:?}", arr);
    println!("Still prints");
}
