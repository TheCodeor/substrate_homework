
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = [5, 2, 8, 3, 1];
    println!("Before sorting: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("After sorting: {:?}", numbers);
}
