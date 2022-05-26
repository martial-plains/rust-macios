use apple_frameworks::foundation::{traits::t_NSArray, NSArray, NSMutableArray, NSNumber};

fn main() {
    let arr = [NSNumber::from(1), NSNumber::from(2), NSNumber::from(3)];

    let nsarr = NSArray::from(&arr[..]);

    let new_arr =
        unsafe { nsarr.arrayByAddingObjectsFromArray(NSArray::from(vec![NSNumber::from(4)])) };

    for i in &new_arr {
        println!("{}", i);
    }
}
