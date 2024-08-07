use std::{collections::HashMap, fmt::Debug};

pub fn k_way_external_merge_sort<T: Ord + Debug>(
    mut v: Vec<T>,
    k: usize,
    page_size: usize,
) -> Vec<T> {
    assert!(k > 0);
    assert!(page_size > 0);

    // println!("run (k, page_size): ({}, {})", k, page_size);
    let mut n = 0;
    loop {
        v = pass_n(v, k, page_size, n);
        // println!("pass_{} {:?}", n, v);
        let pass_item_count = get_pass_n_page_count(k, n) * page_size;
        if pass_item_count >= v.len() {
            break;
        }
        n += 1;
    }

    v
}

fn pass_n<T: Ord + Debug>(mut v: Vec<T>, k: usize, page_size: usize, n: usize) -> Vec<T> {
    let mut result = Vec::with_capacity(v.len()); // non in-memory implementation: acts as the output file

    let pass_page_count = get_pass_n_page_count(k, n);

    while !v.is_empty() {
        // non in-memory implementation: while not EOF
        let mut pages;
        (pages, v) = build_n_next_pages(v, pass_page_count, page_size);

        if n == 0 {
            for page in &mut pages {
                page.sort();
            }
        }

        let mut page_groups = build_page_groups(pages, k);
        let mut local_result = vec![];

        loop {
            // get reference of first available value of each group
            let mut local_pages: HashMap<(usize, usize), &T> = HashMap::new();

            for (group_id, group) in page_groups.iter().enumerate() {
                for (page_id, page) in group.iter().enumerate() {
                    if page.is_empty() {
                        continue;
                    }

                    local_pages.insert((group_id, page_id), page.first().unwrap());
                }
            }

            // this happens when all groups are already empty => exit clause, wev'e dealt with all the data
            if local_pages.is_empty() {
                break;
            }

            // get group id and page id of the minimum value
            let ((group_id, page_id), _) = local_pages
                .into_iter()
                .fold(None, |acc, entry| match acc {
                    None => Some(entry),
                    Some(previous) => {
                        if entry.1 < previous.1 {
                            Some(entry)
                        } else {
                            Some(previous)
                        }
                    }
                })
                .unwrap();

            // remove minimum local value from located page and add it to the result
            let minimum_local_value = page_groups[group_id][page_id].remove(0);
            local_result.push(minimum_local_value);
        }

        result.extend(local_result); // non in-memory implementation: write to a file instead
    }

    result
}

/**
 * Take a list of pages as input and split them in k complete groups
 */
fn build_page_groups<T: Ord + Debug>(mut pages: Vec<Vec<T>>, k: usize) -> Vec<Vec<Vec<T>>> {
    assert!(pages.len() % k == 0);

    let group_count = pages.len() / k;
    let mut groups = Vec::with_capacity(group_count);

    for _ in 0..group_count {
        let mut group = vec![];
        for _ in 0..k {
            group.push(pages.remove(0));
        }
        groups.push(group);
    }

    groups
}

fn build_n_next_pages<T: Ord + Debug>(
    mut v: Vec<T>,
    n: usize,
    page_size: usize,
) -> (Vec<Vec<T>>, Vec<T>) {
    let mut pages = Vec::with_capacity(n);
    for _ in 0..n {
        let current_page = v.drain(..page_size.min(v.len())).collect();
        pages.push(current_page);
    }
    (pages, v)
}

fn get_pass_n_page_count(k: usize, n: usize) -> usize {
    k * 2_usize.pow(n as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_sorted() {
        let original = vec![5, 9, 5, 2, 5, 4, 0, 9, 1, 3];
        let is_sorted = |v: Vec<i32>| v.windows(2).all(|w| w[0] <= w[1]);

        assert!(is_sorted(k_way_external_merge_sort(original.clone(), 2, 3)));
        assert!(is_sorted(k_way_external_merge_sort(original.clone(), 3, 3)));
        assert!(is_sorted(k_way_external_merge_sort(original.clone(), 2, 1)));
        assert!(is_sorted(k_way_external_merge_sort(original.clone(), 1, 5)));
        assert!(is_sorted(k_way_external_merge_sort(original.clone(), 1, 1)));
    }

    #[test]
    fn build_n_next_pages() {
        let original = vec![5, 9, 5, 2, 5, 4, 0, 9, 1, 3];
        let (pages, rest) = super::build_n_next_pages(original, 2, 2);

        assert_eq!(vec![vec![5, 9], vec![5, 2]], pages);
        assert_eq!(vec![5, 4, 0, 9, 1, 3], rest);
    }

    #[test]
    fn build_page_groups() {
        let pages = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
            vec![13, 14, 15],
            vec![16, 17, 18],
        ];

        assert_eq!(
            vec![
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![vec![10, 11, 12], vec![13, 14, 15], vec![16, 17, 18]]
            ],
            super::build_page_groups(pages, 3) // which means that each group should contain 3 pages
        );
    }
}
