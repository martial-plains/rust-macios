use rust_macios::natural_language::nl_tag_scheme;

fn main() {
    println!("{}", unsafe { nl_tag_scheme::Script.clone() });
}
