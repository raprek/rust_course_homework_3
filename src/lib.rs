// task 1
pub fn get_tuple_el<T>(in_tuple: &mut (T, T), last: bool) -> &mut T {
    if last {
        &mut in_tuple.1
    } else {
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
pub fn get_nth_from_tail<T>(in_slice: &[T], n: usize) -> Option<&T> {
    if in_slice.len() > n {
        Option::Some(&in_slice[in_slice.len() - n - 1])
    } else {
        Option::None
    }
}

// task 4
pub fn split_slice_by_nth<T>(in_slice: &[T], n: usize) -> Option<(&[T], &[T])> {
    if in_slice.len() > n {
        Option::Some((&in_slice[0..n], &in_slice[n..]))
    } else {
        Option::None
    }
}

// task 5
pub fn split_slice_by_four_slices<T>(in_slice: &[T]) -> [Option<&[T]>; 4] {
    let mut res: [Option<&[T]>; 4] = [None; 4];
    let step = if in_slice.len() % 4 == 0 {
        in_slice.len() / 4
    } else {
        in_slice.len() % 4
    };

    let mut i = 0;
    while step * i < in_slice.len() {
        let end = step * (i + 1);
        res[i] = if end > in_slice.len() {
            Some(&in_slice[step * i..])
        } else {
            Some(&in_slice[step * i..end])
        };
        i += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tuple_by_bool() {
        let mut in_tuple = (1, 2);
        assert_eq!(get_tuple_el(&mut in_tuple, false).to_owned(), in_tuple.0);
        assert_eq!(get_tuple_el(&mut in_tuple, true).to_owned(), in_tuple.1);
    }

    #[test]
    fn test_get_slice_item() {
        let slice = &mut [1, 2, 3];
        assert_eq!(get_mut(slice, 1).unwrap().to_owned(), slice[1]);
        assert_eq!(get_mut(slice, 10), Option::None)
    }

    #[test]
    fn test_get_nth_from_tail() {
        let slice = &[1, 2, 3];
        assert_eq!(get_nth_from_tail(slice, 0).unwrap().to_owned(), slice[2]);
        assert_eq!(get_nth_from_tail(slice, 1).unwrap().to_owned(), slice[1]);
        assert_eq!(get_nth_from_tail(slice, 2).unwrap().to_owned(), slice[0]);
        assert_eq!(get_nth_from_tail(slice, 3), Option::None)
    }

    #[test]
    fn test_split_slice_by_nth() {
        let slice = &[1, 2, 3];
        assert_eq!(
            split_slice_by_nth(slice, 0).unwrap().to_owned(),
            (&slice[..0], &slice[0..])
        );
        assert_eq!(
            split_slice_by_nth(slice, 1).unwrap().to_owned(),
            (&slice[..1], &slice[1..])
        );
        assert_eq!(
            split_slice_by_nth(slice, 2).unwrap().to_owned(),
            (&slice[..2], &slice[2..])
        );
        assert_eq!(split_slice_by_nth(slice, 3), Option::None)
    }

    #[test]
    fn test_split_slice_by_four_slices() {
        // test not equal parts
        let slice = &[1, 2, 3, 4, 5, 6];
        let [res_p1, res_p2, res_p3, res_p4] = split_slice_by_four_slices(slice);
        assert_eq!(res_p1.unwrap(), &slice[0..2]);
        assert_eq!(res_p2.unwrap(), &slice[2..4]);
        assert_eq!(res_p3.unwrap(), &slice[4..6]);
        assert_eq!(res_p4, None);

        // test one element
        let slice = &[1];
        let [res_p1, res_p2, res_p3, res_p4] = split_slice_by_four_slices(slice);
        assert_eq!(res_p1.unwrap(), &slice[..]);
        assert_eq!(res_p2, None);
        assert_eq!(res_p3, None);
        assert_eq!(res_p4, None);

        // test equal parts
        let slice = &[1, 2, 3, 4, 5, 6, 7, 8];
        let [res_p1, res_p2, res_p3, res_p4] = split_slice_by_four_slices(slice);
        assert_eq!(res_p1.unwrap(), &slice[0..2]);
        assert_eq!(res_p2.unwrap(), &slice[2..4]);
        assert_eq!(res_p3.unwrap(), &slice[4..6]);
        assert_eq!(res_p4.unwrap(), &slice[6..8]);
    }
}
