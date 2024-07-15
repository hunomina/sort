mod k_way_external_merge_sort;

use k_way_external_merge_sort::k_way_external_merge_sort;

fn main() {
    let original = vec![5, 9, 5, 2, 5, 4, 0, 9, 1, 3];
    println!("original {:?}", original);
    println!(
        "sorted 2;3 {:?}",
        k_way_external_merge_sort(original.clone(), 2, 3)
    );
    println!("-----------------------------------------");
    println!(
        "sorted 3;3 {:?}",
        k_way_external_merge_sort(original.clone(), 3, 3)
    );

    println!("-----------------------------------------");
    println!(
        "sorted 2;1 {:?}",
        k_way_external_merge_sort(original.clone(), 2, 1)
    );

    println!("-----------------------------------------");
    println!("sorted 1;5 {:?}", k_way_external_merge_sort(original, 1, 5));
}
