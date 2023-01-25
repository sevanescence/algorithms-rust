mod merge_sort;

use merge_sort::merge_sort;

fn main() {
    let v = vec![5, 2, 3, 7, 8, 4, 6, 9, 1];

    println!("---- Merge Sort ----");
    println!("{:?}", merge_sort(v));
    println!("--- End of Merge ---");
}
