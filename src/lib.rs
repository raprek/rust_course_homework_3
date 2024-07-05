use std::ops::Index;

// task 1
pub fn get_tuple_el<T>(in_tuple: &mut (T, T), last: bool) -> &mut T {
    if last {
        &mut in_tuple.1
    }
    else {
        &mut in_tuple.0
    }
}

// task 2
pub fn get_mut<T>(in_slice: &mut [T], n: usize) -> Option<&mut T> {
    if in_slice.len() > n {
        Option::Some(&mut in_slice[n])
    } else {
        Option::None
    }
}

// task 3
pub fn get_mut_from_tail<T>(in_slice: &mut [T], n: usize) -> Option<&mut T> {
    if in_slice.len() >= n + 1 {
        Option::Some(&mut in_slice[in_slice.len() - n - 1])
    } else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tuple_by_bool() {
        let mut in_tuple = (1,2);
        assert_eq!(get_tuple_el(&mut in_tuple, false).to_owned(), in_tuple.0);
        assert_eq!(get_tuple_el(&mut in_tuple, true).to_owned(), in_tuple.1);
    }

    #[test]
    fn test_get_slice_item() {
        let slice = &mut [1,2,3];
        assert_eq!(get_mut(slice, 1).unwrap().to_owned(), slice[1]);
        assert_eq!(get_mut(slice, 10), Option::None)
    }

    #[test]
    fn test_get_mut_from_tail() {
        let slice = &mut [1,2,3];
        assert_eq!(get_mut_from_tail(slice, 0).unwrap().to_owned(), slice[2]);
        assert_eq!(get_mut_from_tail(slice, 1).unwrap().to_owned(), slice[1]);
        assert_eq!(get_mut_from_tail(slice, 2).unwrap().to_owned(), slice[0]);
        assert_eq!(get_mut_from_tail(slice, 3), Option::None)
    }
}