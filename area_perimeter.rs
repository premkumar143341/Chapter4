fn calculate_area_perimeter(length: u32, width: u32) -> (u32, u32) {
    let area = length * width;
    let perimeter = 2 * (length + width);
    (area, perimeter)
}

fn main() {
    let (area, perimeter) = calculate_area_perimeter(5, 3);
    println!("Area: {}, Perimeter: {}", area, perimeter);
}
