// Textbook recursive implementation

// Fully generic version that takes anything that can be compared in both

fn _find<T: PartialEq + PartialOrd, A: AsRef<[T]>>(
    array: A,
    key: T,
    index: usize,
) -> Option<usize> {
    let array = array.as_ref();
    if array.len() == 1 {
        if array[0] == key {
            return Some(index);
        } else {
            return None;
        }
    }

    let mid = array.len() / 2;
    if array[mid] > key {
        return _find(&array[..mid], key, index);
    } else {
        return _find(&array[mid..], key, index + mid);
    }
}

pub fn find<T: PartialEq + PartialOrd, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }
    _find(array, key, 0)
}

//
// This takes anything that can be converted to [i32]
//
// fn _find(array: &[i32], key: i32, index: usize) -> Option<usize> {
//     if array.len() == 1 {
//         if array[0] == key {
//             return Some(index);
//         } else {
//             return None;
//         }
//     }
//     let mid = array.len() / 2;
//     if array[mid] > key {
//         return _find(&array[..mid], key, index);
//     } else {
//         return _find(&array[mid..], key, index + mid);
//     }
// }
//
// pub fn find<T: AsRef<[i32]>>(array: T, key: i32) -> Option<usize> {
//     let array = array.as_ref();
//     if array.is_empty() {
//         return None;
//     }
//     _find(array, key, 0)
// }
//

//
// This only takes [i32] and nothing else.
//
// fn _find(array: &[i32], key: i32, index: usize) -> Option<usize> {
//     if array.len() == 1 {
//         if array[0] == key {
//             return Some(index);
//         } else {
//             return None;
//         }
//     }
//
//     let mid = array.len() / 2;
//     if array[mid] > key {
//         return _find(&array[..mid], key, index);
//     } else {
//         return _find(&array[mid..], key, index + mid);
//     }
// }
//
// pub fn find(array: &[i32], key: i32) -> Option<usize> {
//     if array.is_empty() {
//         return None;
//     }
//     _find(array, key, 0)
// }
//
