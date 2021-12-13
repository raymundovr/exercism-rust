#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/* My original solution was this:
fn _is_sublist<T: PartialEq>(list_a: &[T], list_b: &[T]) -> bool {
    let first = list_a.first().unwrap();

    for (index, value) in list_b.iter().enumerate() {
        if value == first {
            let mut matches = 0;
            let mut index_a = 0;
            for i in index..cmp::min(list_b.len(), index + list_a.len()) {
                if list_a[index_a] == list_b[i] {
                    matches += 1;
                }
                index_a += 1;
            }
            if matches == list_a.len() {
                return true;
            }
        }
    }

    false
}
*/

/* Then I learned about .windows() */
fn is_sublist<T: PartialEq>(list_a: &[T], list_b: &[T]) -> bool {
    list_b.windows(list_a.len()).any(|chunk| chunk == list_a)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    // Empty cases
    if first_list.len() == 0 && second_list.len() > 0 {
        return Comparison::Sublist;
    }

    if first_list.len() > 0 && second_list.len() == 0 {
        return Comparison::Superlist;
    }

    // Regular cases
    if first_list == second_list {
        return Comparison::Equal;
    }

    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }
 
    Comparison::Unequal
}
