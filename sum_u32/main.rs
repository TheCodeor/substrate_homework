fn main() {
    let numbers = vec![1, 2, 3, 5, 5];
    match sum_u32(&numbers) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occurred"),
    }
}

fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut total: u32 = 0;
    for &num in numbers {
        total = total.checked_add(num)?;
    }
    Some(total)
}
