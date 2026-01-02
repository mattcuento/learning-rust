use std::collections::HashMap;

fn main() {
    // Median tests
    let list1 = vec![1, 2, 3];
    assert_eq!(median(&list1), 2.0);
    let list2: Vec<i32> = vec![];
    assert_eq!(median(&list2), 0.0);
    let list3 = vec![1, 2, 3, 4, 5];
    assert_eq!(median(&list3), 3.0);
    let list4 = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(median(&list4), 3.5);

    // Mode tests
    let list5 = vec![1, 2, 2, 3, 3, 3, 4];
    assert_eq!(mode(&list5), 3);
    let list6: Vec<i32> = vec![];
    assert_eq!(mode(&list6), 0);
    let list7 = vec![5, 5, 5, 1, 1, 2];
    assert_eq!(mode(&list7), 5);
}

fn median(int_list: &[i32]) -> f64 {
    if int_list.is_empty() {
        return 0.0;
    }
    let mut sorted_list = int_list.to_vec();
    sorted_list.sort();

    let mid = sorted_list.len() / 2;
    if sorted_list.len() % 2 == 0 {
        (sorted_list[mid - 1] + sorted_list[mid]) as f64 / 2.0
    } else {
        sorted_list[mid] as f64
    }
}

fn mode(int_list: &[i32]) -> i32 {
    if int_list.is_empty() {
        return 0;
    }

    let mut counts = HashMap::new();
    for &value in int_list {
        *counts.entry(value).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .unwrap_or(0)
}
