use rust_macios::foundation::NSProcessInfo;

fn main() {
    let thermal_state = NSProcessInfo::process_info().thermal_state();

    println!("{thermal_state:?}");
}
