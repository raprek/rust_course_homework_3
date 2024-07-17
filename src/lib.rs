// task 1
pub fn get_tuple_el<T>(in_tuple: &mut (T, T), last: bool) -> &mut T {
    if last {
        &mut in_tuple.1
    } else {
        &mut in_tuple.0
    }
}

// task 2
pub fn get_mut<T>(in_slice: &mut [T], n: usize) -> &mut T {
    &mut in_slice[n]
}

// task 3
pub fn get_nth_from_tail<T>(in_slice: &[T], n: usize) -> &T {
    &in_slice[in_slice.len() - n - 1]
}

// task 4
pub fn split_slice_by_nth<T>(in_slice: &[T], n: usize) -> (&[T], &[T]) {
    (&in_slice[0..n], &in_slice[n..])
}

// task 5
pub fn split_slice_by_four_slices<T>(in_slice: &[T]) -> [&[T]; 4] {
    let mut res: [&[T]; 4] = [&[]; 4];
    let (k, m) = (in_slice.len() / 4, in_slice.len() % 4);
    for i  in 0..4 {
        res[i] = &in_slice[i*k+std::cmp::min(i, m)..(i+1)*k + std::cmp::min(i + 1, m)];
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
        assert_eq!(get_mut(slice, 1).to_owned(), slice[1]);
        assert_eq!(get_mut(slice, 2).to_owned(), slice[2]);
    }

    #[test]
    #[should_panic]
    fn test_get_slice_item_invalid_input() {
        get_mut(&mut [1, 2, 3], 10);
    }

    #[test]
    fn test_get_nth_from_tail() {
        let slice = &[1, 2, 3];
        assert_eq!(get_nth_from_tail(slice, 0).to_owned(), slice[2]);
        assert_eq!(get_nth_from_tail(slice, 1).to_owned(), slice[1]);
        assert_eq!(get_nth_from_tail(slice, 2).to_owned(), slice[0]);
    }

    #[test]
    #[should_panic]
    fn test_get_nth_from_tail_invalid_input() {
        get_nth_from_tail(&[1, 2, 3], 3);
    }



    #[test]
    fn test_split_slice_by_nth() {
        let slice = &[1, 2, 3];
        assert_eq!(
            split_slice_by_nth(slice, 0).to_owned(),
            (&slice[..0], &slice[0..])
        );
        assert_eq!(
            split_slice_by_nth(slice, 1).to_owned(),
            (&slice[..1], &slice[1..])
        );
        assert_eq!(
            split_slice_by_nth(slice, 2).to_owned(),
            (&slice[..2], &slice[2..])
        );
        assert_eq!(split_slice_by_nth(&[1, 2, 3], 3), (&slice[..], &slice[3..]));
    }

    #[test]
    fn test_split_slice_by_four_slices() {
        // test empty slice
        let slice: &[i32; 0] = &[];
        let [res_p1, res_p2, res_p3, res_p4] = split_slice_by_four_slices(slice);
        assert_eq!(res_p1, []);
        assert_eq!(res_p2, []);
        assert_eq!(res_p3, []);
        assert_eq!(res_p4, []);


        // test not equal parts
        let slice = &[1, 2, 3, 4, 5, 6];
        let [res_p1, res_p2, res_p3, res_p4] = split_slice_by_four_slices(slice);
        assert_eq!(res_p1, &slice[0..2]);
        assert_eq!(res_p2, &slice[2..4]);
        assert_eq!(res_p3, &slice[4..5]);
        assert_eq!(res_p4, &slice[5..]);

        // test one element
        let slice = &[1];
        let [res_p1, res_p2, res_p3, res_p4] = split_slice_by_four_slices(slice);
        assert_eq!(res_p1, &slice[..]);
        assert_eq!(res_p2, []);
        assert_eq!(res_p3, []);
        assert_eq!(res_p4, []);

        // test equal parts
        let slice = &[1, 2, 3, 4, 5, 6, 7, 8];
        let [res_p1, res_p2, res_p3, res_p4] = split_slice_by_four_slices(slice);
        assert_eq!(res_p1, &slice[0..2]);
        assert_eq!(res_p2, &slice[2..4]);
        assert_eq!(res_p3, &slice[4..6]);
        assert_eq!(res_p4, &slice[6..8]);

        // test equal parts
        let slice: &[usize; 18] = &core::array::from_fn(|i| i + 1);
        let [res_p1, res_p2, res_p3, res_p4] = split_slice_by_four_slices(slice);
        assert_eq!(res_p1, &slice[0..5]);
        assert_eq!(res_p2, &slice[5..10]);
        assert_eq!(res_p3, &slice[10..14]);
        assert_eq!(res_p4, &slice[14..]);
    }
}
