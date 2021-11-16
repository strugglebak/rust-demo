fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // We didn’t need to make v1_iter mutable when we used a for loop
    // because the loop took ownership of v1_iter and made it mutable behind the scenes.
    for v in v1_iter {
        println!("Got: {}", v);
    }

    let v2: Vec<i32> = vec![1, 2, 3];
    // error:
    // iterator adaptors are lazy, and we need to consume the iterator here
    // v2.iter().map(|x| x + 1);
    // use collect to consume the iterator
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();

    assert_eq!(v3, vec![1, 2, 3]);
}
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}
