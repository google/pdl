/// Placeholder implementation of Vec::drain_filter.
/// The feature drain_filter is currently unstable.
pub fn drain_filter<T, F>(input: &mut Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    // Pass 1: compute the total number of removed elements.
    let mut total_left_count = 0;
    for element in input.iter() {
        total_left_count += !predicate(element) as usize;
    }
    // Pass 2: compute the final position of each element in the input
    // array in order to position left elements first and drained elements
    // last, preserving the order.
    let mut rank = Vec::with_capacity(input.len());
    let mut left_count = 0;
    let mut removed_count = 0;
    for element in input.iter() {
        if predicate(element) {
            rank.push(total_left_count + removed_count);
            removed_count += 1;
        } else {
            rank.push(left_count);
            left_count += 1;
        }
    }
    // Pass 3: swap the elements to their final position.
    let mut n = 0;
    while n < input.len() {
        let rank_n = rank[n];
        if n != rank_n {
            input.swap(n, rank_n);
            rank.swap(n, rank_n);
        } else {
            n += 1;
        }
    }
    // Finally: split off the removed elements off the input vector.
    input.split_off(total_left_count)
}

#[cfg(test)]
mod test {
    use crate::utils::drain_filter;

    #[test]
    fn test_drain_filter() {
        let mut input = vec![1, 4, 2, 5, 3, 6, 7];
        let drained = drain_filter(&mut input, |element| *element > 3);
        assert_eq!(input, vec![1, 2, 3]);
        assert_eq!(drained, vec![4, 5, 6, 7]);
    }
}
