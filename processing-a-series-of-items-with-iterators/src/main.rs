fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // We didn’t need to make v1_iter mutable when we used a for loop
    // because the loop took ownership of v1_iter and made it mutable behind the scenes.
    for v in v1_iter {
        println!("Got: {}", v);
    }
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
}
