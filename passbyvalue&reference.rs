fn pass_by_value(mut a: i32) {
    a += 10;
    println!("Inside pass_by_value: {}", a);
}

fn pass_by_reference(b: &mut i32) {
    *b += 10;
    println!("Inside pass_by_reference: {}", b);
}

fn main() {
    let x = 5;
    println!("Before pass_by_value: {}", x);
    pass_by_value(x);
    println!("After pass_by_value: {}", x);

    let mut y = 5;
    println!("\nBefore pass_by_reference: {}", y);
    pass_by_reference(&mut y);
    println!("After pass_by_reference: {}", y);
}
