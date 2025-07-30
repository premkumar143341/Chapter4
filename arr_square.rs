fn arr_square(arr: &[i32]) -> Vec<i32> {
    arr.iter().map(|x| x * x).collect()
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let squared = arr_square(&numbers);
    println!("Original: {:?}", numbers);
    println!("Squared: {:?}", squared);
}
