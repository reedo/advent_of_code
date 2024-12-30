fn all_numbers_are_close(list: &[usize]) -> bool {
    let mut prev = list[0];
    for curr in list[1..].iter() {
        let diff = curr.abs_diff(prev);
        if (1..=3).contains(&diff) {
            prev = *curr;
            continue;
        } else {
            return false;
        }
    }

    true
}

#[allow(clippy::ptr_arg)]
pub fn list_is_safe(list: &Vec<usize>) -> bool {
    let is_ascending = list.is_sorted_by(PartialOrd::gt);
    let is_descending = list.is_sorted_by(PartialOrd::lt);

    (is_ascending || is_descending) && all_numbers_are_close(list)
}
