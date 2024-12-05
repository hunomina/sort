/**
 * Bubble sort:
 * Complexity: O(n^2)
 * Compares items two by two, progressively "bubbling" (=pushing) bigger items at the end of the array
 */

pub fn bubble_sort<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    for i in 0..v.len() {
        for j in 0..v.len() - i - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
            }
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_sorted() {
        let is_sorted = |v: Vec<i32>| v.windows(2).all(|w| w[0] <= w[1]);

        let original = vec![5, 9, 5, 2, 5, 4, 0, 9, 1, 3];

        assert!(is_sorted(bubble_sort(original)));
    }
}
