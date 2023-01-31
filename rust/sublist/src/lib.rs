#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    }

    if first_list.is_empty() {
        return Comparison::Sublist;
    }

    if second_list.is_empty() {
        return Comparison::Superlist;
    }

    if first_list.len() == second_list.len() {
        if first_list == second_list {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    }

    if first_list.len() < second_list.len() {
        if second_list
            .windows(first_list.len())
            .any(|w| w == first_list)
        {
            return Comparison::Sublist;
        }
    } else {
        if first_list
            .windows(second_list.len())
            .any(|w| w == second_list)
        {
            return Comparison::Superlist;
        }
    }

    return Comparison::Unequal;
}
